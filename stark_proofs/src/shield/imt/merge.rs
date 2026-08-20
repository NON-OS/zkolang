// NONOS Operating System (AGPL-3.0-or-later)

use super::leaf::Leaf;
use super::order::cmp;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// The chain as a subtree sees it: every leaf, in order.
pub(crate) type State = Vec<Leaf>;

pub(crate) fn same(a: &State, b: &State) -> bool {
    a.len() == b.len()
        && a.iter().zip(b).all(|(x, y)| {
            cmp(&x.value, &y.value) == Ordering::Equal
                && cmp(&x.next_value, &y.next_value) == Ordering::Equal
                && x.next_index == y.next_index
                && x.is_last == y.is_last
        })
}

/// What one subtree attests: the chain it started from and the chain it left.
pub(crate) struct Range {
    pub old: State,
    pub new: State,
}

/// Merge two subtree ranges.
///
/// One equality, not a list of shapes it recognises. A disjunction of accept
/// cases fails closed on the topology nobody drew: it refuses valid work, every
/// refusal test still passes, and only a positive case shows it. That happened
/// here once already, and to the disjointness precondition before it.
///
/// So B started from the chain A left, or there is no merge. Separate gaps, a
/// stitched seam and the case where A's last key points at the leaf B starts from
/// all satisfy it without being named; two subtrees that both started from the
/// pre-batch chain do not, which is the double update.
pub(crate) fn stitch(a: &Range, b: &Range) -> Option<Range> {
    if !same(&a.new, &b.old) {
        return None;
    }
    Some(Range { old: a.old.clone(), new: b.new.clone() })
}
