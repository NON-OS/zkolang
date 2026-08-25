// NONOS Operating System (AGPL-3.0-or-later)

use super::pack::pack;
use super::uf::Uf;
use crate::crypto::stark::air::GpGroup;
use alloc::vec::Vec;

/// A running product over k wired columns carries degree k+1, so one group over
/// the whole trace width costs as much evaluation domain as it saves in columns.
/// Eight keeps the fused degree at the level the rest of the AIR already sets.
const CAP: usize = 8;

fn cell(g: &GpGroup, pos: usize, width: usize) -> usize {
    let k = g.wired_cols.len();
    (pos / k) * width + g.wired_cols[pos % k]
}

/// Each group holds its own cells equal, so the conjunction of all of them is the
/// transitive closure of those equalities. The closure is what gets re-cut, into
/// groups narrow enough to keep the degree down.
pub(crate) fn collapse(gps: &[GpGroup], span: usize, width: usize) -> Vec<GpGroup> {
    pack(closure(gps, span, width), span, width, CAP)
}

fn closure(gps: &[GpGroup], span: usize, width: usize) -> Vec<Vec<usize>> {
    let n = span * width;
    let mut uf = Uf::new(n);
    for g in gps {
        for (pos, &img) in g.sigma.iter().enumerate() {
            if pos != img {
                uf.union(cell(g, pos, width), cell(g, img, width));
            }
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
