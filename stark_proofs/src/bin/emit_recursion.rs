// NONOS Operating System (AGPL-3.0-or-later)
//! Emit a recursion proof and write it out.
//!
//! Not a test. A test proves and drops the proof on the floor; this writes the
//! bytes a verifier is given, reads them back, and verifies from the bytes
//! rather than from the value it just held in memory. A proof that only
//! verifies in the process that made it has not been shown to travel.

use stark_proofs::crypto::stark::air::{
    deserialize_proof_ext, serialize_proof_ext, stark_prove_ext, stark_verify_ext, Air,
};
use stark_proofs::recursion_assembly::{assemble, assemble_real, Tamper};
use std::time::Instant;

const N_QUERIES: usize = 32;
const BLOWUP: u32 = 8;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "recursion.proof".into());
    // "real" asks for the recursion over the deployed join-split; anything
    // else keeps the fixture inner, which stays as the regression shape.
    let real = std::env::args().nth(2).as_deref() == Some("real");

    let t0 = Instant::now();
    let asm = if real { assemble_real(Tamper::None) } else { assemble(Tamper::None) };
    let built = t0.elapsed();
    println!(
        "assembly  width={} log_trace_len={} degree={} transitions={} groups={}",
        asm.wired.trace_width(),
        asm.wired.log_trace_len(),
        asm.wired.constraint_degree(),
        asm.wired.num_transition(),
        asm.n_groups
    );
    println!("assembled in {:?}", built);

    let t1 = Instant::now();
    let proof = stark_prove_ext(&asm.wired, &asm.witness, N_QUERIES, BLOWUP);
    let proved = t1.elapsed();
    println!("proved in {:?}", proved);

    let bytes = serialize_proof_ext(&proof);
    std::fs::write(&out, &bytes).expect("write proof");
    println!("wrote {} bytes to {out}", bytes.len());

    // Verify from the bytes, not from `proof`. The round trip is the point: a
    // proof that only checks against the value still in memory has not been
    // shown to survive being written down and handed over.
    let read = deserialize_proof_ext(&bytes).expect("the proof we just wrote did not parse");
    let t2 = Instant::now();
    let ok = stark_verify_ext(&asm.wired, &read, N_QUERIES, BLOWUP);
    println!("verified from disk in {:?}: {}", t2.elapsed(), ok);

    if !ok {
        eprintln!("the emitted proof did not verify");
        std::process::exit(1);
    }
}
