// NONOS Operating System (AGPL-3.0-or-later)

use super::read::absorbed_at;
use crate::crypto::stark::air::{Cell, RATE};
use crate::shield::join::publics::{NF0, NF1, OUT_CM0, OUT_CM1};
use crate::shield::wire_class::{pair, Class};
use alloc::vec::Vec;

/// The lanes a node's effect is made of, in the order it holds them: two
/// retired notes, then two created ones.
pub(crate) const LANES: usize = 4 * RATE;

/// Tie the effect the node composes to the words its child's transcript
/// absorbed.
///
/// `read_effect` locates the cells; this is what makes reading them the only
/// option. The node holds its effect one word per row from `eff_off`, the same
/// shape the publics region uses, and each row joins the transcript cell that
/// absorbed that word. Without the join the effect is a free witness and a node
/// verifies one child while composing another's move, every proof still
/// verifying. `base` is where the child intent's words start.
pub(crate) fn effect_classes(l: usize, eff_off: usize, base: usize) -> Vec<Class> {
    let mut g = Vec::with_capacity(LANES);
    for (k, word) in words(base).enumerate() {
        let (row, col) = absorbed_at(l, word);
        g.push(pair(eff_off + k, 0, row, col));
    }
    g
}

/// The public word index of each effect lane, nullifiers before outputs.
pub(crate) fn words(base: usize) -> impl Iterator<Item = usize> {
    [NF0, NF1, OUT_CM0, OUT_CM1]
        .into_iter()
        .flat_map(move |d| (0..RATE).map(move |c| base + d + c))
}

/// The cells a set of classes names, for arguing about what got tied.
pub(crate) fn cells(classes: &[Class]) -> Vec<Cell> {
    classes.iter().flatten().copied().collect()
}
