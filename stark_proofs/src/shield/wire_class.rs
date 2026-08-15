// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Cell, GpGroup};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// A set of cells a binding forces equal. The bindings emit these rather than
/// grand product groups, so the mechanism that enforces them can change without
/// touching what they mean.
pub(crate) type Class = Vec<Cell>;

pub(crate) fn pair(ra: usize, ca: usize, rb: usize, cb: usize) -> Class {
    alloc::vec![Cell { row: ra, col: ca }, Cell { row: rb, col: cb }]
}

/// One class as its own grand product, which is what the assembly does today.
/// Swapping this for a single permutation over every class is the shrink; the
/// classes and their forgeries do not move.
pub(crate) fn group_of(span: usize, class: &Class) -> GpGroup {
    let mut cols: Vec<usize> = class.iter().map(|c| c.col).collect();
    cols.sort_unstable();
    cols.dedup();
    let k = cols.len();
    let mut sigma: Vec<usize> = (0..span * k).collect();
    for w in class.windows(2) {
        let ia = cols.iter().position(|&c| c == w[0].col).unwrap();
        let ib = cols.iter().position(|&c| c == w[1].col).unwrap();
        sigma.swap(w[0].row * k + ia, w[1].row * k + ib);
    }
    GpGroup { wired_cols: cols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}

/// Every class in one product over the whole trace: identity on all columns with
/// each class rotated into a cycle. Thirteen hundred running products and their
/// periodic columns become one product and two columns per trace column.
///
/// Disjointness is a precondition. A class over a cell that already carries an
/// image rewrites its cycle and drops the earlier binding, silently, while every
/// other binding still holds. Proven in lean/Zkolang/Wiring.lean.
pub(crate) fn global_group(span: usize, width: usize, classes: &[Class]) -> GpGroup {
    let cols: Vec<usize> = (0..width).collect();
    let mut sigma: Vec<usize> = (0..span * width).collect();
    for class in classes {
        if class.len() < 2 {
            continue;
        }
        let idx: Vec<usize> = class.iter().map(|c| c.row * width + c.col).collect();
        let first = sigma[idx[0]];
        for w in idx.windows(2) {
            sigma[w[0]] = sigma[w[1]];
        }
        sigma[idx[idx.len() - 1]] = first;
    }
    GpGroup { wired_cols: cols, sigma, beta: Fp::from_u64(5), gamma: Fp::from_u64(7) }
}
