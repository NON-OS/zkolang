// NONOS Operating System (AGPL-3.0-or-later)

use super::uf::Uf;
use crate::crypto::stark::air::GpGroup;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

fn cell(g: &GpGroup, pos: usize, width: usize) -> usize {
    let k = g.wired_cols.len();
    (pos / k) * width + g.wired_cols[pos % k]
}

/// Each group holds its own cells equal, so the conjunction of all of them is the
/// transitive closure of those equalities. One sigma over the full width carries
/// that same closure, and cells outside every class stay identity and cancel.
pub(crate) fn collapse(gps: &[GpGroup], span: usize, width: usize) -> GpGroup {
    let n = span * width;
    let mut uf = Uf::new(n);
    for g in gps {
        for (pos, &img) in g.sigma.iter().enumerate() {
            if pos != img {
                uf.union(cell(g, pos, width), cell(g, img, width));
            }
        }
    }
    let mut sigma: Vec<usize> = (0..n).collect();
    let mut head = alloc::vec![usize::MAX; n];
    let mut prev = alloc::vec![usize::MAX; n];
    for c in 0..n {
        let r = uf.find(c);
        if head[r] == usize::MAX {
            head[r] = c;
        } else {
            sigma[prev[r]] = c;
        }
        prev[r] = c;
    }
    for r in 0..n {
        if head[r] != usize::MAX && prev[r] != head[r] {
            sigma[prev[r]] = head[r];
        }
    }
    GpGroup {
        wired_cols: (0..width).collect(),
        sigma,
        beta: Fp::from_u64(5),
        gamma: Fp::from_u64(7),
    }
}
