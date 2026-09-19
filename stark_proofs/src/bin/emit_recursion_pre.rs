// NONOS Operating System (AGPL-3.0-or-later)
//! Emit an outer artifact through the preprocessed-periodic prover.
//!
//! The difference from `emit_recursion` is the only thing that matters here.
//! That binary proves with `stark_prove_ext_blown`, which leaves the periodic
//! evaluations at the out-of-domain point outside the proof, so a chain
//! verifier has to be handed them. Every value a verifier is handed rather
//! than derives is a value an adversary chooses, and the composition at z is
//! the last one in the settlement path that was still supplied.
//!
//! `stark_prove_ext_preprocessed` commits the periodic columns to their own
//! Merkle root, carries the claims at z in a sidecar, and opens the wide
//! periodic row against that root at every consistency query. The verifier
//! then computes the composition from proof bytes alone and checks the
//! periodic inputs against a root baked into the contract at deployment.
//!
//! Which outer is proved comes from the point selector: the settlement outer
//! by default, or `transfer` with `queries=N` for the recursion over a
//! transfer inner. The outer's own parameters are the deployment set under
//! `deployment` and the development set otherwise, whichever inner sits under
//! it.
//!
//! The periodic tree is a constant of the circuit and is kept: `.treecache`
//! beside the output holds every level under the root's name, and a run that
//! finds its tree there skips the commitment phase, which was half of a
//! settlement proof. `root=<hex>` names the tree to load and the root the
//! proof must verify against; a tree whose own root differs is refused
//! before anything is baked. Without `root=` the tree is built, stored, and
//! the proof verified against the root it produced.
//!
//! The bytes are written through the same encoder the reference vector uses,
//! and read back and verified from disk rather than from the value still in
//! memory, because a proof that only verifies in the process that produced it
//! has not been shown to travel.

use stark_proofs::crypto::stark::air::{
    stark_prove_ext_preprocessed_tree, stark_verify_ext_preprocessed_pub, Air,
};
use stark_proofs::proof_wire::serialize_pre;
use stark_proofs::recursion_assembly::point::Point;
use stark_proofs::recursion_assembly::{assemble, Tamper};
use stark_proofs::shield_params::{deployment, dev};
use stark_proofs::tree_cache;
use std::time::Instant;

/// Sixty four lower case hex characters, or nothing. A root of any other
/// shape is a typo, and a typo baked into a verifier is a verifier for no
/// proof.
fn well_formed_root(hex: &str) -> Option<String> {
    let hex = hex.trim().to_ascii_lowercase();
    if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    Some(hex)
}

fn hex_of(root: &[u8; 32]) -> String {
    root.iter().map(|b| format!("{b:02x}")).collect()
}

/// Four limbs as the 256 bit word the pool carries them in, limb 0 lowest,
/// as a 0x hex string.
fn pack_u256(limbs: &[stark_proofs::crypto::stark::field::Fp]) -> String {
    let mut v: u128 = 0;
    let mut hi: u128 = 0;
    for (i, l) in limbs.iter().enumerate() {
        let x = l.to_u64() as u128;
        match i {
            0 => v |= x,
            1 => v |= x << 64,
            2 => hi |= x,
            _ => hi |= x << 64,
        }
    }
    format!("0x{hi:032x}{v:032x}")
}

/// The public words regrouped as the pool presents them: per intent, six
/// digests, four scalars and the recipient, eleven words, in SPEC section 6
/// order. This is the list a verifier's `publicsOf(batch)` is computed from,
/// emitted beside the flat limb list it must equal so both sides of that
/// check come from one file.
fn pool_intents(publics: &[stark_proofs::crypto::stark::field::Fp]) -> Vec<String> {
    use stark_proofs::shield::join::publics::{
        ASSET_ID, CLEARING_PRICE, FEE, PUBLIC_AMOUNT, RECIPIENT, WORDS,
    };
    publics
        .chunks_exact(WORDS)
        .map(|w| {
            let mut words: Vec<String> = Vec::with_capacity(11);
            for d in 0..6 {
                words.push(pack_u256(&w[4 * d..4 * d + 4]));
            }
            for i in [PUBLIC_AMOUNT, FEE, ASSET_ID, CLEARING_PRICE] {
                words.push(format!("0x{:x}", w[i].to_u64()));
            }
            words.push(pack_u256(&w[RECIPIENT..RECIPIENT + 4]));
            format!(
                "{{\"words\": [{}]}}",
                words
                    .iter()
                    .map(|s| format!("\"{s}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let point = Point::from_args(&args);
    let real = args.iter().any(|a| a == "real") || point != Point::Settlement;
    let deploy = args.iter().any(|a| a == "deployment");
    let supplied = args.iter().find_map(|a| a.strip_prefix("root="));
    /*
     * The output path is the first argument that is not a flag, so every flag
     * has to be named here or it becomes the filename.
     */
    let out = args
        .iter()
        .find(|a| {
            !Point::is_flag(a)
                && a.as_str() != "real"
                && a.as_str() != "deployment"
                && !a.starts_with("root=")
        })
        .cloned()
        .unwrap_or_else(|| format!("{}-pre.proof", point.name()));

    let expected_root = match supplied {
        Some(hex) => match well_formed_root(hex) {
            Some(hex) => Some(hex),
            None => {
                eprintln!("root= wants 64 hex characters; refusing to bake {hex:?}");
                std::process::exit(2);
            }
        },
        None => None,
    };

    let (n_queries, grind_bits, extra_blowup) = if deploy {
        (
            deployment::N_QUERIES,
            deployment::GRIND_BITS,
            deployment::EXTRA_BLOWUP_BITS,
        )
    } else {
        (dev::N_QUERIES, dev::GRIND_BITS, dev::EXTRA_BLOWUP_BITS)
    };
    println!(
        "point     {} over an inner at {} queries, extra blowup {}",
        point.name(),
        point.inner_queries(),
        point.inner_extra()
    );
    println!(
        "params    {} queries, {} grind bits, extra blowup {} (rate 1/{})",
        n_queries,
        grind_bits,
        extra_blowup,
        1usize << (1 + extra_blowup)
    );

    /*
     * The tree, if a previous run left it. Loaded before the assembly so the
     * file's cost is paid while nothing else is resident, and so a run that
     * will skip the commitment says so on its first line rather than its
     * hundredth.
     */
    let cache_dir = tree_cache::dir_beside(&out);
    let cached = expected_root.as_deref().and_then(|hex| {
        let path = tree_cache::entry(&cache_dir, hex);
        let t = Instant::now();
        let tree = tree_cache::load(&path);
        match &tree {
            Some(tree) => println!(
                "tree      loaded {} leaves from {} in {:?}",
                tree.len(),
                path.display(),
                t.elapsed()
            ),
            None => println!(
                "tree      none at {}; the commitment will be built",
                path.display()
            ),
        }
        tree
    });
    let loaded = cached.is_some();

    let t0 = Instant::now();
    let mut asm = if real {
        match point.assemble_wired(Point::emit_wiring()) {
            Ok(asm) => asm,
            Err(why) => {
                eprintln!("{why}");
                std::process::exit(2);
            }
        }
    } else {
        assemble(Tamper::None)
    };
    println!(
        "assembly  width={} log_trace_len={} degree={} transitions={} groups={}",
        asm.wired.trace_width(),
        asm.wired.log_trace_len(),
        asm.wired.constraint_degree(),
        asm.wired.num_transition(),
        asm.n_groups
    );
    println!("assembled in {:?}", t0.elapsed());

    /*
     * The statement's public inputs, absorbed into the transcript before
     * anything else so the proof is a proof about them. They are written
     * beside the proof as plain words, because a verifier that mirrors the
     * absorb needs the exact list and not a description of it.
     */
    println!("publics   {} words absorbed first", asm.publics.len());
    let publics_json = format!(
        "{{\n  \"point\": \"{}\",\n  \"artifact\": \"{out}\",\n  \"n_publics\": {},\n  \
         \"absorb\": \"transcript label, then each word as absorb_fp in order, then the trace root\",\n  \
         \"publics\": [{}],\n  \
         \"words_per_intent\": {},\n  \
         \"pool_words\": \"per intent, SPEC section 6 order: six digests, public_amount, fee, \
         asset_id, clearing_price, recipient; a digest and the recipient pack four limbs as \
         limb0 + limb1 * 2^64 + limb2 * 2^128 + limb3 * 2^192\",\n  \
         \"intents\": [\n    {}\n  ]\n}}\n",
        point.name(),
        asm.publics.len(),
        asm.publics
            .iter()
            .map(|v| v.to_u64().to_string())
            .collect::<Vec<_>>()
            .join(", "),
        stark_proofs::shield::join::publics::WORDS,
        pool_intents(&asm.publics).join(",\n    ")
    );
    let publics_out = format!("{out}.publics.json");

    let t2 = Instant::now();
    let Some((pre, tree)) = stark_prove_ext_preprocessed_tree(
        &asm.wired,
        &asm.witness,
        n_queries,
        grind_bits,
        extra_blowup,
        &asm.publics,
        cached,
        None,
    ) else {
        /*
         * Only a watcher can cancel, and this binary passes none, so reaching
         * here means the prover's own contract changed underneath it. Say so
         * and leave nothing on disk rather than carrying on with a hole.
         */
        eprintln!("the prover returned no proof without being asked to stop");
        std::process::exit(1);
    };
    println!("proved in {:?}", t2.elapsed());
    println!(
        "sidecar   {} periodic claims at z, {} openings",
        pre.periodic_z.len(),
        pre.openings.len()
    );

    /*
     * The witness is read by the prover and by nothing after it. Off the heap
     * before the tree is written, which is the next large thing this process
     * touches.
     */
    drop(core::mem::take(&mut asm.witness));

    let root = tree.root();
    let root_hex = hex_of(&root);
    println!("periodic root {root_hex}");
    if let Some(expected) = &expected_root {
        if *expected != root_hex {
            eprintln!(
                "the tree's root is {root_hex} and root= said {expected}; this outer at this \
                 rate does not have that root, refusing to write a proof against it"
            );
            std::process::exit(1);
        }
    }

    if !loaded {
        let path = tree_cache::entry(&cache_dir, &root_hex);
        let t = Instant::now();
        match tree_cache::store(&path, &tree) {
            Ok(()) => println!(
                "tree      stored at {} in {:?}",
                path.display(),
                t.elapsed()
            ),
            Err(e) => eprintln!("tree      not stored at {}: {e}", path.display()),
        }
    }
    drop(tree);

    let t3 = Instant::now();
    let ok = stark_verify_ext_preprocessed_pub(
        &asm.wired,
        &pre,
        n_queries,
        grind_bits,
        extra_blowup,
        &root,
        &asm.publics,
    );
    println!(
        "verified against the baked root and the publics in {:?}: {}",
        t3.elapsed(),
        ok
    );
    if !ok {
        eprintln!("the proof did not verify against its own root; nothing written");
        std::process::exit(1);
    }

    let bytes = serialize_pre(&pre);
    std::fs::write(&out, &bytes).expect("write proof");
    println!("wrote {} bytes to {out}", bytes.len());
    std::fs::write(&publics_out, &publics_json).expect("write publics");
    println!("wrote {} publics to {publics_out}", asm.publics.len());
}
