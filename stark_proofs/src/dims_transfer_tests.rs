// What a transfer costs, at the tree depth the pool deploys.
//
// The roundtrip gate builds a minimal instance, which is right for checking that
// a forgery rejects through the permutation and wrong for any statement about
// what a spend costs. This runs the deployed depth and proves it.
use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air};
use crate::shield::key::Break;
use crate::shield::test::scenario::{balanced, balanced_deployed};

fn shape(tag: &str, js: &crate::shield::join::JoinSplit) {
    let w = &js.wired;
    let (tw, deg, lt) = (w.trace_width(), w.constraint_degree(), w.log_trace_len());
    let t = 1usize << lt;
    let n = (2usize * (deg.max(1) * t).next_power_of_two()) << 3;
    let np = w.periodic_columns().len();
    let gb = |c: usize| (c as u128 * n as u128 * 8) / (1024 * 1024 * 1024);
    std::eprintln!(
        "{tag} trace_width={tw} degree={deg} log_trace_len={lt} t={t} n_periodic={np} \
         eval_domain={n} trace_lde_GB={} periodic_lde_GB={} publics={}",
        gb(tw),
        gb(np),
        js.intent.len()
    );
}

#[test]
#[ignore]
fn probe_transfer_dims() {
    shape("MINIMAL", &balanced(Break::None));
    shape("DEPLOYED", &balanced_deployed(Break::None));
}

/// The numbers that belong in any claim about transfer cost: what it takes to
/// make one, what it takes to check one, and how big the thing is on the wire.
#[test]
#[ignore]
fn probe_deployed_transfer_proves() {
    use crate::crypto::stark::air::serialize_proof_ext;
    let js = balanced_deployed(Break::None);
    shape("DEPLOYED", &js);

    let t0 = std::time::Instant::now();
    let proof = stark_prove_ext(&js.wired, &js.witness, 32, 8);
    let prove = t0.elapsed();

    let bytes = serialize_proof_ext(&proof);

    let t1 = std::time::Instant::now();
    let ok = stark_verify_ext(&js.wired, &proof, 32, 8);
    let verify = t1.elapsed();

    std::println!(
        "TRANSFER prove_ms={} verify_ms={} proof_bytes={} queries=32 blowup=8",
        prove.as_millis(),
        verify.as_millis(),
        bytes.len()
    );
    assert!(ok, "the deployed transfer did not verify");
}
