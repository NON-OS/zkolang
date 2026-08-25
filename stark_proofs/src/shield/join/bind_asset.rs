// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::Cell;
use crate::shield::wire_class::Class;
use alloc::vec::Vec;

/// Limb two of a note commitment is its asset. The balance row sums values and
/// knows nothing of assets, and the public asset word is pinned to the first input
/// alone, so without this the other three notes carry any asset they like into the
/// same total and value crosses between them.
///
/// A transfer is one asset in and the same asset out. A swap is not, and when that
/// path opens conservation has to run per asset rather than over a single sum; this
/// class is the transfer's answer and does not generalise to it.
pub(crate) fn asset_classes(l: &Layout) -> Vec<Class> {
    const ASSET_LIMB: usize = 2;
    if l.note.len() < 2 {
        return Vec::new();
    }
    alloc::vec![l.note.iter().map(|&n| Cell { row: n, col: ASSET_LIMB }).collect()]
}
