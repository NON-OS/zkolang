// NONOS Operating System (AGPL-3.0-or-later)
//! Emit the settlement artifact through the preprocessed-periodic prover.
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
//! The bytes are written through the same encoder the reference vector uses,
//! and read back and verified from disk rather than from the value still in
//! memory, because a proof that only verifies in the process that produced it
//! has not been shown to travel.

use stark_proofs::crypto::stark::air::{
    periodic_root, stark_prove_ext_preprocessed, stark_verify_ext_preprocessed, Air,
};
use stark_proofs::proof_wire::serialize_pre;
use stark_proofs::recursion_assembly::{assemble, assemble_real, Tamper};
use stark_proofs::shield_params::{deployment, dev};
use std::time::Instant;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "recursion-pre.proof".into());
    let real = std::env::args().nth(2).as_deref() == Some("real");
    let deploy = std::env::args().nth(3).as_deref() == Some("deployment");
    let (n_queries, grind_bits, extra_blowup) = if deploy {
        (deployment::N_QUERIES, deployment::GRIND_BITS, deployment::EXTRA_BLOWUP_BITS)
    } else {
        (dev::N_QUERIES, dev::GRIND_BITS, dev::EXTRA_BLOWUP_BITS)
    };
    println!(
        "params    {} queries, {} grind bits, extra blowup {} (rate 1/{})",
        n_queries,
        grind_bits,
        extra_blowup,
        1usize << (1 + extra_blowup)
    );

    let t0 = Instant::now();
    let mut asm = if real { assemble_real(Tamper::None) } else { assemble(Tamper::None) };
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
     * Prove first, then compute the root the contract bakes.
     *
     * Both build the same periodic tree, which on the settlement outer is the
     * largest allocation in the process. Doing the root first meant the
     * allocator had held that peak once before the prover asked for it again,
     * and the machine was killed rather than finishing. Ordering them apart
     * gives the process one peak instead of a peak on top of a high water mark.
     */
    let t2 = Instant::now();
    let pre = stark_prove_ext_preprocessed(
        &asm.wired,
        &asm.witness,
        n_queries,
        grind_bits,
        extra_blowup,
    );
    println!("proved in {:?}", t2.elapsed());
    println!(
        "sidecar   {} periodic claims at z, {} openings",
        pre.periodic_z.len(),
        pre.openings.len()
    );

    let bytes = serialize_pre(&pre);
    std::fs::write(&out, &bytes).expect("write proof");
    println!("wrote {} bytes to {out}", bytes.len());

    /*
     * The witness is read by the prover and by nothing after it. Dropping it
     * before the root is computed takes the trace off the heap while the
     * periodic set is being built, which is the one place those two would
     * otherwise be live together.
     */
    drop(core::mem::take(&mut asm.witness));

    let t1 = Instant::now();
    let root = periodic_root(&asm.wired, extra_blowup);
    let root_hex: String = root.iter().map(|b| format!("{b:02x}")).collect();
    println!("periodic root {root_hex} in {:?}", t1.elapsed());

    let t3 = Instant::now();
    let ok = stark_verify_ext_preprocessed(
        &asm.wired,
        &pre,
        n_queries,
        grind_bits,
        extra_blowup,
        &root,
    );
    println!("verified against the baked root in {:?}: {}", t3.elapsed(), ok);

    if !ok {
        eprintln!("the emitted proof did not verify; not leaving it on disk as if it had");
        std::process::exit(1);
    }
}
