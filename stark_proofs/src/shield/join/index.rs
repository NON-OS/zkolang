// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, IndexScalar};
use crate::crypto::stark::field::Fp;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Positions {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    /// Row carrying the recovered index, one per spent note.
    pub value_row: Vec<usize>,
    pub bits: usize,
}

/// The pool proves a note's position through its path directions; the nullifier
/// hashes that position as a scalar. One of these per spent note recovers the
/// scalar from bits the assembly then binds to those directions, so the two are
/// the same position rather than two numbers that happen to agree.
pub(crate) fn positions(leaves: &[usize], depth: usize) -> Positions {
    let mut regions: Vec<Box<dyn AirExt>> = Vec::with_capacity(leaves.len());
    let mut traces = Vec::with_capacity(leaves.len());
    let mut value_row = Vec::with_capacity(leaves.len());
    for &leaf in leaves {
        let region = IndexScalar::new(depth, leaf as u64);
        traces.push(region.trace());
        value_row.push(region.value_row());
        regions.push(Box::new(region));
    }
    Positions { regions, traces, value_row, bits: depth }
}
