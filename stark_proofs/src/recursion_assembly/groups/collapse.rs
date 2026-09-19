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

/// The wiring itself: the classes of cells the binds hold equal, before any
/// decision about how to argue them.
///
/// `collapse` cuts these into groups that fit the degree budget, and that
/// cut is an implementation choice rather than a statement about the trace.
/// Anything that changes how the permutation is argued, in particular
/// replacing many narrow arguments with one chained one, has to leave this
/// partition exactly as it is, so it is exposed and gated on rather than
/// reachable only through the packer. A cell is `row * width + column`.
///
/// Canonically ordered, because the union-find walks cells in whatever order
/// the binds arrive and a gate compares two runs of two builders: cells
/// ascending within a class, classes ascending by their least cell.
pub fn wiring_classes(gps: &[Bind], span: usize, width: usize) -> Vec<Vec<usize>> {
    let mut classes = closure(gps, span, width);
    for class in classes.iter_mut() {
        class.sort_unstable();
    }
    classes.sort_unstable_by_key(|c| c[0]);
    classes
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
