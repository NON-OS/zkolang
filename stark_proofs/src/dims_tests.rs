// dims probe: measure the step assembly's shape without proving.
use crate::crypto::stark::air::Air;
use crate::recursion_assembly::{assemble_step, Tamper};

#[test]
fn probe_dims() {
    let asm = assemble_step(Tamper::None);
    let w = &asm.wired;
    let tw = w.trace_width();
    let deg = w.constraint_degree();
    let lt = w.log_trace_len();
    let t = 1usize << lt;
    let bound = (deg.max(1) * t).next_power_of_two();
    let n = (2usize * bound) << 3; // extra_blowup_bits = 3
    let ld = (n as u64).trailing_zeros();
    let est_gb = (tw as u128 * n as u128 * 8) / (1024 * 1024 * 1024);
    std::eprintln!(
        "PROBE trace_width={} degree={} log_trace_len={} t={} n_periodic={} n_groups={} log_dn={} eval_domain={} est_trace_lde_GB={}",
        tw, deg, lt, t, w.periodic_columns().len(), asm.n_groups, ld, n, est_gb
    );
    std::eprintln!("PROBE region_offsets={:?} span={}", asm.region_offsets, asm.lay.span);
}
