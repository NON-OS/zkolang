// dims probe: the join-split assembly's shape at the deployed round count.
use crate::crypto::stark::air::Air;
use crate::recursion_assembly::{assemble, Tamper};

#[test]
#[ignore]
fn probe_join_split_dims() {
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
    std::eprintln!(
        "JOINSPLIT trace_width={tw} degree={deg} log_trace_len={lt} t={t} n_periodic={np} n_groups={} n_q={} eval_domain={n} trace_lde_GB={trace_gb} periodic_lde_GB={periodic_gb} total_GB={}",
        asm.n_groups,
        asm.lay.n_q,
        trace_gb + periodic_gb
    );
}
