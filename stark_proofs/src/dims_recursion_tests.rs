// dims probe: the RECURSION assembly, the verifier that checks a join-split proof
// inside a proof. This is not the transfer circuit; see dims_transfer_tests.
use crate::crypto::stark::air::Air;
use crate::recursion_assembly::{assemble, Tamper};

#[test]
#[ignore]
fn probe_recursion_dims() {
    let asm = assemble(Tamper::None);
    let w = &asm.wired;
    let tw = w.trace_width();
    let deg = w.constraint_degree();
    let lt = w.log_trace_len();
    let t = 1usize << lt;
    let bound = (deg.max(1) * t).next_power_of_two();
    let n = (2usize * bound) << 3;
    let np = w.periodic_columns().len();
    let trace_gb = (tw as u128 * n as u128 * 8) / (1024 * 1024 * 1024);
    let periodic_gb = (np as u128 * n as u128 * 8) / (1024 * 1024 * 1024);
    let nr = asm.region_offsets.len();
    let zeros: usize = w
        .periodic_columns()
        .iter()
        .map(|c| c.iter().filter(|v| **v == crate::crypto::stark::field::Fp::ZERO).count())
        .sum();
    let cells = np * (1usize << lt);
    std::eprintln!(
        "REGIONS n={nr} selectors={nr} region_periodic={} per_region={:.1} zero_frac={:.4}",
        np - nr,
        (np - nr) as f64 / nr as f64,
        zeros as f64 / cells as f64
    );
    std::eprintln!(
        "RECURSION trace_width={tw} degree={deg} log_trace_len={lt} t={t} n_periodic={np} n_groups={} n_q={} eval_domain={n} trace_lde_GB={trace_gb} periodic_lde_GB={periodic_gb} total_GB={}",
        asm.n_groups,
        asm.lay.n_q,
        trace_gb + periodic_gb
    );
}
