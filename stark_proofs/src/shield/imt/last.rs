// NONOS Operating System (AGPL-3.0-or-later)

use super::leaf::Leaf;
use super::order::cmp;
use core::cmp::Ordering;

/// Exactly one leaf is last, and it holds the largest key.
///
/// A second one, or one in the middle, makes every key above it look excluded:
/// the range check stops at the low bound and never applies an upper one. That is
/// a non-membership proof for a member, which is a double spend. The magic
/// maximum carried this implicitly; the flag has to carry it out loud.
pub(crate) fn last_is_the_maximum(leaves: &[Leaf]) -> bool {
    let mut last = None;
    for (i, l) in leaves.iter().enumerate() {
        if l.is_last {
            if last.is_some() {
                return false;
            }
            last = Some(i);
        }
    }
    match last {
        None => false,
        Some(i) => {
            // Its own next is unused, so it must be zero rather than whatever the
            // witness felt like putting there.
            leaves[i].next_value.iter().all(|v| v.value() == 0)
                && leaves
                    .iter()
                    .enumerate()
                    .all(|(j, l)| j == i || cmp(&l.value, &leaves[i].value) == Ordering::Less)
        }
    }
}
