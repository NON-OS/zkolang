// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::GpGroup;
use crate::shield::join::publics::{CLEARING_PRICE, WORDS};
use crate::shield::wire::equate;
use alloc::vec::Vec;

/// settleBatch caps a batch at this many intents.
pub(crate) const MAX_INTENTS: usize = 64;

/// A batch clears at one price. Without this each intent carries its own price
/// and a settler prices fills against each other, which is the ordering leak the
/// uniform clearing exists to remove.
///
/// `pub_off` is the first row of each intent's publics region, in batch order.
pub(crate) fn price_uniform(span: usize, pub_off: &[usize]) -> Vec<GpGroup> {
    let mut g = Vec::new();
    let first = match pub_off.first() {
        Some(o) => o + CLEARING_PRICE,
        None => return g,
    };
    for o in pub_off.iter().skip(1) {
        g.push(equate(span, alloc::vec![0], &[(first, 0, o + CLEARING_PRICE, 0)]));
    }
    let _ = WORDS;
    g
}
