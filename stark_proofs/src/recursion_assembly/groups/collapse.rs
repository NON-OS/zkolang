// NONOS Operating System (AGPL-3.0-or-later)

use super::helpers::Bind;
use super::pack::pack;
use super::uf::Uf;
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

/// A running product over k wired columns carries degree k+1, so one group over
/// the whole trace width costs as much evaluation domain as it saves in columns.
/// Eight keeps the fused degree at the level the rest of the AIR already sets.
const CAP: usize = 8;

/// Each group holds its own cells equal, so the conjunction of all of them is the
/// transitive closure of those equalities. The closure is what gets re-cut, into
/// groups narrow enough to keep the degree down.
pub fn collapse(gps: &[Bind], span: usize, width: usize) -> Vec<GpGroup> {
    pack(closure(gps, span, width), span, width, CAP)
}

fn closure(gps: &[Bind], span: usize, width: usize) -> Vec<Vec<usize>> {
    let n = span * width;
    let mut uf = Uf::new(n);
    for g in gps {
        for &(ra, ia, rb, ib) in &g.swaps {
            uf.union(ra * width + g.wired_cols[ia], rb * width + g.wired_cols[ib]);
        }
    }
    let mut seen = alloc::vec![usize::MAX; n];
    let mut out: Vec<Vec<usize>> = Vec::new();
    for c in 0..n {
        let r = uf.find(c);
        if seen[r] == usize::MAX {
            seen[r] = out.len();
            out.push(Vec::new());
        }
        out[seen[r]].push(c);
    }
    out.retain(|class| class.len() >= 2);
    out
}
