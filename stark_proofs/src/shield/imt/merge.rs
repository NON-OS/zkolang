// NONOS Operating System (AGPL-3.0-or-later)

use super::order::cmp;
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// What a subtree does to the chain: the pointers it sets, and the range it owns.
///
/// `low` is the pre-batch leaf its first key follows. Two subtrees whose ranges
/// fall in the same gap both start from that leaf, and both would set its pointer
/// if they were merged as written.
pub(crate) struct Range {
    pub low: [Fp; RATE],
    /// Every `(holder, points_at)` this subtree writes, in chain order.
    pub sets: Vec<([Fp; RATE], [Fp; RATE])>,
}

impl Range {
    pub fn first(&self) -> Option<[Fp; RATE]> {
        self.sets.first().map(|(_, to)| *to)
    }

    pub fn last(&self) -> Option<[Fp; RATE]> {
        self.sets.last().map(|(from, _)| *from)
    }
}

/// Merge two adjacent subtree ranges into one.
///
/// The seam is the whole problem. Both were computed against the pre-batch tree,
/// so if they share a gap they each believe they own the low leaf's pointer. Left
/// alone, the second overwrites the first and that subtree's whole range leaves
/// the chain, while the product still closes and every other binding still holds.
///
/// So: no leaf may be written by both, and what A's last key points at has to be
/// what B starts from. That second condition is children-chain wearing the linked
/// list's clothes, A.new == B.old carried across the seam.
pub(crate) fn stitch(a: &Range, b: &Range) -> Option<Range> {
    let (last, first) = match (a.last(), b.first()) {
        (Some(l), Some(f)) => (l, f),
        _ => return None,
    };
    if cmp(&last, &first) != Ordering::Less {
        return None;
    }
    // A leaf written twice is a range dropped.
    for (from, _) in &a.sets {
        if b.sets.iter().any(|(other, _)| cmp(from, other) == Ordering::Equal) {
            return None;
        }
    }
    // Sharing a gap means both believe they own the same low leaf's pointer. B
    // either follows A's last key or starts from a leaf A never touched; starting
    // where A started is the double update.
    if cmp(&a.low, &b.low) == Ordering::Equal {
        return None;
    }
    let mut sets = a.sets.clone();
    sets.extend(b.sets.iter().copied());
    Some(Range { low: a.low, sets })
}
