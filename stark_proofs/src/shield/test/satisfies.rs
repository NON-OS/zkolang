// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Air, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(super) fn satisfies(air: &WiredMultiExt, witness: &[Fp]) -> bool {
    let w = air.trace_width();
    let ws = air.window_size();
    let total = 1usize << air.log_trace_len();
    let periodic = air.periodic_columns();
    for r in 0..total - (ws - 1) {
        let mut window = Vec::with_capacity(ws * w);
        for k in 0..ws {
            window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
        }
        let per: Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
        if air.transition(&window, &per).iter().any(|v| *v != Fp::ZERO) {
            return false;
        }
    }
    air.boundary().into_iter().all(|(col, row, val)| witness[row * w + col] == val)
}
