// NONOS Operating System (AGPL-3.0-or-later)
//! The real outer's shape, from the full query assembly: the structure a
//! settlement verifier derives its constants from, the baked periodic root it
//! holds, and a satisfaction walk over the whole witness so the shape shipped is
//! a shape that provably accepts. No proving here; the proof artifact is the
//! emitter's job. This is the contract side's re-gate input for the real
//! circuit, produced whole or not at all.
//!
//! By default the outer is assembled over the settlement inner, 32 queries at
//! rate 1/16, the point the registered keys hold. With `transfer` it is assembled
//! over a transfer inner at rate 1/4, 56 queries unless `queries=N` says
//! otherwise, which is what the outer would have to verify if senders proved at
//! that point. Run at 56 and at 64 and the emitted layout, rather than an
//! estimate, says what adopting the transfer point costs the recursion and
//! whether the 144 bit count still fits the same trace budget: the outer's span
//! and log trace length are the numbers to read.
//!
//! The outer's authentication regions size the inner's evaluation domain from
//! `inner::extra()`, which the environment sets. A transfer assembly is therefore
//! only consistent under `NONOS_INNER_EXTRA=1`, and the emit refuses to run the
//! transfer mode under any other value rather than emit a shape that does not
//! match the proof it was built over.

use stark_proofs::crypto::stark::air::Air;
use stark_proofs::recursion_assembly::build::assemble_over;
use stark_proofs::recursion_assembly::{assemble_real, inner, Tamper};
use stark_proofs::shield_params::transfer;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let at_transfer = args.iter().any(|a| a == "transfer");
    /*
     * The query count for a transfer assembly. The transfer point's own count is
     * the default; `queries=N` overrides it so the outer can be assembled at more
     * than one count on the same rate and grind, which is how "does 64 still fit
     * the 2^21 budget" becomes a measurement instead of a model.
     */
    let queries = args
        .iter()
        .find_map(|a| a.strip_prefix("queries=").and_then(|v| v.parse::<usize>().ok()))
        .unwrap_or(transfer::N_QUERIES);
    let out = args
        .iter()
        .find(|a| a.as_str() != "transfer" && !a.starts_with("queries="))
        .cloned()
        .unwrap_or_else(|| {
            if at_transfer {
                format!("transfer-{queries}-structure.json")
            } else {
                "real-structure.json".into()
            }
        });
    let point = if at_transfer { "transfer" } else { "settlement" };

    /*
     * The blowup the outer's authentication assumes for the inner comes from the
     * environment. A transfer inner is proved at rate 1/4, so the environment
     * must say so, or the outer would authenticate a 4x domain as a 16x one and
     * the shape emitted would not be the shape of any proof that verifies.
     */
    if at_transfer && inner::extra() != transfer::EXTRA_BLOWUP_BITS {
        eprintln!(
            "the transfer assembly needs NONOS_INNER_EXTRA={} (found {}); refusing to emit \
             a shape that does not match its inner",
            transfer::EXTRA_BLOWUP_BITS,
            inner::extra()
        );
        std::process::exit(2);
    }

    let h = inner::hasher();
    let t0 = Instant::now();
    let asm = if at_transfer {
        let inner_at = inner::shield_join_split_at(
            &h,
            queries,
            transfer::GRIND_BITS,
            transfer::EXTRA_BLOWUP_BITS,
        );
        assemble_over(&h, inner_at, Tamper::None, usize::MAX)
    } else {
        assemble_real(Tamper::None)
    };
    eprintln!("assembled {point} in {:?}", t0.elapsed());

    let t1 = Instant::now();
    let ok = stark_proofs::witness_satisfies_public(&asm.wired, &asm.witness);
    eprintln!("satisfies in {:?}: {ok}", t1.elapsed());
    if !ok {
        eprintln!("the full-coverage assembly does not satisfy; refusing to emit its shape");
        std::process::exit(1);
    }

    /*
     * The numbers a re-gate compares first, on one line so they cannot be missed.
     * The evaluation domain is 2 * next_pow2(degree * 2^log_trace_len), so the
     * log trace length alone fixes it only while the degree stays at or under 16.
     * The degree is printed beside it rather than assumed, and the domain is
     * derived here from both, so a degree that moved underneath a confirmed log
     * length cannot arrive looking like a sizing success.
     */
    let degree = asm.wired.constraint_degree();
    let log_trace_len = asm.wired.log_trace_len();
    let log_domain = (2 * (degree << log_trace_len).next_power_of_two()).trailing_zeros();
    println!(
        "outer     point={point} span={} log_trace_len={log_trace_len} degree={degree} \
         log_domain={log_domain} n_queries={} trace_width={}",
        asm.lay.span,
        asm.lay.n_q,
        asm.wired.trace_width()
    );

    let js = stark_proofs::shield_deployed_wired();
    let root_extra = if at_transfer { transfer::EXTRA_BLOWUP_BITS } else { inner::extra() };
    let root = stark_proofs::crypto::stark::air::periodic_root_poseidon(&js, root_extra, &h);
    let root_hex: String = root
        .iter()
        .map(|l| format!("{:016x}", l.to_u64()))
        .collect::<Vec<_>>()
        .join("");

    let json = format!(
        "{{\n  \"point\": \"{}\",\n  \"log_trace_len\": {},\n  \"trace_width\": {},\n  \
         \"num_transition\": {},\n  \"num_groups\": {},\n  \"constraint_degree\": {},\n  \
         \"inner_log_trace_len\": {},\n  \"inner_trace_width\": {},\n  \"n_queries\": {},\n  \
         \"periodic_root_poseidon\": \"{}\"\n}}\n",
        point,
        asm.wired.log_trace_len(),
        asm.wired.trace_width(),
        asm.wired.num_transition(),
        asm.n_groups,
        asm.wired.constraint_degree(),
        asm.lay.t_inner.trailing_zeros(),
        asm.lay.width_inner,
        asm.lay.n_q,
        root_hex,
    );
    std::fs::write(&out, &json).expect("write structure");
    println!("{json}");
    println!("wrote {out}");

    /*
     * The region layout, for a verifier whose evaluators are config-driven over
     * it: every Layout scalar, the region bases, and the permutation's committed
     * geometry. Same discipline as the shape file: derived, gated by the
     * satisfaction walk above, never typed.
     */
    let lay = &asm.lay;
    /*
     * The outer's own periodic root is the one allocation in this section that
     * scales with the outer: it extends every periodic column to the evaluation
     * domain before hashing. It is computed first and announced on both sides,
     * so if the machine cannot hold it the run says where it died, rather than
     * ending after the shape with no layout and no message.
     */
    eprintln!("computing the outer periodic root");
    let outer_root_hex: String = {
        let r = stark_proofs::crypto::stark::air::periodic_root(&asm.wired, 0);
        r.iter().map(|b| format!("{b:02x}")).collect()
    };
    eprintln!("outer periodic root done");
    eprintln!("reading the permutation columns");
    let (sel_idx, row_idx, sig_base) = asm.wired.permutation_columns();
    eprintln!("permutation columns read; formatting {} groups", sig_base.len());
    let groups_json: Vec<String> = asm
        .wired
        .group_params()
        .iter()
        .zip(&sig_base)
        .map(|((cols, beta, gamma), sb)| {
            format!(
                "{{\"wired_cols\": {:?}, \"beta\": {}, \"gamma\": {}, \"sigma_base_col\": {}}}",
                cols,
                beta.to_u64(),
                gamma.to_u64(),
                sb
            )
        })
        .collect();
    let layout = format!(
        "{{\n  \"span\": {},\n  \"l\": {},\n  \"n_q\": {},\n  \"region_offsets\": {:?},\n  \
         \"z_op\": {},\n  \"claim_op\": {},\n  \"deep_coeff_op\": {},\n  \"pub_len\": {},\n  \
         \"ntr\": {},\n  \"ncoeff2\": {},\n  \"n_terms\": {},\n  \"width_inner\": {},\n  \
         \"window_inner\": {},\n  \"depth\": {},\n  \"n_open\": {},\n  \"n_folds\": {},\n  \
         \"log_n_inner\": {},\n  \"pbits\": {},\n  \"fbits\": {},\n  \"t_inner\": {},\n  \
         \"n_pz\": {},\n  \"pa_depth\": {},\n  \"n_chunks\": {},\n  \"frame_len\": {},\n  \
         \"n_coeff\": {},\n  \"c_periodic_col\": {},\n  \"c_z_col\": {},\n  \"c_coeff_col\": {},\n  \
         \"c_comp_z_col\": {},\n  \"sel_col\": {},\n  \"row_col\": {},\n  \
         \"strip_off\": {},\n  \"strip_k\": {},\n  \"strip_echo_width\": {},\n  \
         \"strip_n_out\": {},\n  \"strip_rows\": {},\n  \"outer_periodic_root_keccak\": \"{}\",\n  \
         \"groups\": [\n    {}\n  ]\n}}\n",
        lay.span,
        lay.l,
        lay.n_q,
        asm.region_offsets,
        lay.z_op,
        lay.claim_op,
        lay.deep_coeff_op,
        lay.pub_len,
        lay.ntr,
        lay.ncoeff2,
        lay.n_terms,
        lay.width_inner,
        lay.window_inner,
        lay.depth,
        lay.n_open,
        lay.n_folds,
        lay.log_n,
        lay.pbits,
        lay.fbits,
        lay.t_inner,
        lay.n_pz,
        lay.pa_depth,
        lay.n_chunks,
        lay.frame_len,
        lay.n_coeff,
        lay.c_periodic_col,
        lay.c_z_col,
        lay.c_coeff_col,
        lay.c_comp_z_col,
        sel_idx,
        row_idx,
        lay.strip_off,
        lay.strip_k,
        lay.strip_echo_width,
        lay.strip_n_out,
        lay.strip_rows,
        outer_root_hex,
        groups_json.join(",\n    "),
    );
    let lay_out = out.replace(".json", "-layout.json");
    eprintln!("layout formatted, {} bytes; writing {lay_out}", layout.len());
    std::fs::write(&lay_out, &layout).expect("write layout");
    println!("wrote {lay_out}");
}
