// dims probe: the shield transfer circuit itself, at the deployed tree depth.
// This is the proof a wallet produces for one private transfer. The recursion
// assembly that verifies such a proof inside a proof is a separate, far larger
// shape; see dims_recursion_tests.
use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext, Air};
use crate::shield::key::Break;
use crate::shield::test::scenario::balanced;

#[test]
#[ignore]
fn probe_transfer_dims() {
    let js = balanced(Break::None);
    let w = &js.wired;
    let tw = w.trace_width();
    let deg = w.constraint_degree();
    let lt = w.log_trace_len();
    let t = 1usize << lt;
    let bound = (deg.max(1) * t).next_power_of_two();
    let n = (2usize * bound) << 3;
    let np = w.periodic_columns().len();
    let gb = |c: usize| (c as u128 * n as u128 * 8) / (1024 * 1024 * 1024);
    let proof = stark_prove_ext(w, &js.witness, 32, 8);
    let ok = stark_verify_ext(w, &proof, 32, 8);
    std::eprintln!(
        "TRANSFER trace_width={tw} degree={deg} log_trace_len={lt} t={t} n_periodic={np} \
         eval_domain={n} trace_lde_GB={} periodic_lde_GB={} publics={} verifies={ok}",
        gb(tw),
        gb(np),
        js.intent.len()
    );
}
