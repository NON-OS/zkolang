// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::GpGroup;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// Identity over `cols` with the named cells transposed, so a satisfying grand
/// product forces them equal.
pub(crate) fn equate(
    span: usize,
    cols: Vec<usize>,
    swaps: &[(usize, usize, usize, usize)],
) -> GpGroup {
    let k = cols.len();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for &(ra, ca, rb, cb) in swaps {
        let ia = cols.iter().position(|&c| c == ca).unwrap();
        let ib = cols.iter().position(|&c| c == cb).unwrap();
        sigma.swap(ra * k + ia, rb * k + ib);
    }
    GpGroup { wired_cols: cols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}

pub(crate) fn offsets(rows: &[usize]) -> (Vec<usize>, usize) {
    let mut off = Vec::with_capacity(rows.len());
    let mut r = 0usize;
    for n in rows {
        off.push(r);
        r += n;
    }
    (off, r.next_power_of_two())
}
