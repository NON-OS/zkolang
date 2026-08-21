// NONOS Operating System (AGPL-3.0-or-later)

use super::effect::Effect;
use super::state::Carried;
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::imt::Set;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// The state carry composes transitions; the tree decides which transitions
/// exist. Both are sound alone, and the join between them is a third thing.
///
/// A batch retiring a nullifier already in the set composes perfectly well as an
/// {old, new} chain and cannot be realised by the tree at all: there is no gap to
/// insert into. So the roots a batch exposes have to be the roots the update
/// actually produces.
pub(crate) fn realised(
    h: &Poseidon,
    set: &Set,
    notes: [Fp; RATE],
    index: u64,
    effects: &[Effect],
) -> Option<(Carried, Carried)> {
    let old = Carried {
        note_root: notes,
        next_index: index,
        nullifier_root: set.root(h),
    };

    let mut keys: Vec<[Fp; RATE]> = effects.iter().flat_map(|e| e.nullifiers).collect();
    keys.sort_by(|a, b| crate::shield::imt::cmp(a, b));
    // A repeat is refused by the chain rather than deduplicated here: two spends
    // of one note are two spends, not one.
    for w in keys.windows(2) {
        if crate::shield::imt::cmp(&w[0], &w[1]) != Ordering::Less {
            return None;
        }
    }
    let after = set.insert(&keys)?;

    let mut notes_root = notes;
    for o in effects.iter().flat_map(|e| e.outputs) {
        notes_root = h.compress(&notes_root, &o);
    }
    let new = Carried {
        note_root: notes_root,
        next_index: index + 2 * effects.len() as u64,
        nullifier_root: after.root(h),
    };
    Some((old, new))
}
