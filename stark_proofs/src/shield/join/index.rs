// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{ShieldRegion, IndexScalar};
use crate::crypto::stark::field::Fp;
use crate::shield::key::Break;
use alloc::vec::Vec;

pub struct Positions {
    pub regions: Vec<ShieldRegion>,
    pub traces: Vec<Vec<Fp>>,
    /// Row carrying the recovered index, one per spent note.
    pub value_row: Vec<usize>,
    pub bits: usize,
}

/// The pool proves a note's position through its path directions; the nullifier
/// hashes that position as a scalar. One of these per spent note recovers the
/// scalar from bits the assembly then binds to those directions, so the two are
/// the same position rather than two numbers that happen to agree.
pub fn positions(leaves: &[usize], depth: usize, brk: Break) -> Positions {
    let mut regions: Vec<ShieldRegion> = Vec::with_capacity(leaves.len());
    let mut traces = Vec::with_capacity(leaves.len());
    let mut value_row = Vec::with_capacity(leaves.len());
    for &leaf in leaves {
        // The forgery recovers the sibling position, bit zero flipped, matching
        // the nullifier the same break moves there. Every higher bit is untouched,
        // so they still bind to the membership directions and only bit zero is off.
        // Every honest build passes Break::None and recovers the real position.
        let index = if brk == Break::ForeignIndex0 { (leaf as u64) ^ 1 } else { leaf as u64 };
        let region = IndexScalar::new(depth, index);
        traces.push(region.trace());
        value_row.push(region.value_row());
        regions.push(ShieldRegion::Index(region));
    }
    Positions { regions, traces, value_row, bits: depth }
}
