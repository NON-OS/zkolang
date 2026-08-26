// NONOS Operating System (AGPL-3.0-or-later)
//! Emit one private transfer proof and write it out.
//!
//! This is the proof a sender makes: two notes in, two notes out, membership
//! against a pool of the deployed depth, at the deployed query count. The
//! aggregation proof that settles a whole batch on chain is a different and far
//! larger object; this is the one an ordinary transaction costs.
//!
//! Written and then read back, because a proof is only worth what it is worth
//! to somebody who was not there when it was made. Verifying the value still in
//! memory shows the prover agrees with itself and nothing more.

use stark_proofs::crypto::stark::air::{
    deserialize_proof_ext, serialize_proof_ext, stark_prove_ext, stark_verify_ext, Air,
};
use stark_proofs::shield::key::Break;
use stark_proofs::shield::test::scenario::balanced_deployed;
use std::time::Instant;

const N_QUERIES: usize = 32;
const BLOWUP: u32 = 8;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "transfer.proof".into());

    let t0 = Instant::now();
    let js = balanced_deployed(Break::None);
    println!(
        "instance  trace_width={} log_trace_len={} degree={} periodic={} publics={}",
        js.wired.trace_width(),
        js.wired.log_trace_len(),
        js.wired.constraint_degree(),
        js.wired.periodic_columns().len(),
        js.intent.len()
    );
    println!("built in {:?}", t0.elapsed());

    let t1 = Instant::now();
    let proof = stark_prove_ext(&js.wired, &js.witness, N_QUERIES, BLOWUP);
    let proved = t1.elapsed();
    println!("proved in {proved:?}  ({N_QUERIES} queries, blowup {BLOWUP})");

    let bytes = serialize_proof_ext(&proof);
    std::fs::write(&out, &bytes).expect("write proof");
    println!("wrote {} bytes to {out}", bytes.len());

    let read = deserialize_proof_ext(&bytes).expect("the proof we just wrote did not parse");
    let t2 = Instant::now();
    let ok = stark_verify_ext(&js.wired, &read, N_QUERIES, BLOWUP);
    println!("verified from disk in {:?}: {ok}", t2.elapsed());

    if !ok {
        eprintln!("the emitted transfer proof did not verify");
        std::process::exit(1);
    }
}
