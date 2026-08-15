// NONOS Operating System (AGPL-3.0-or-later)

use super::limbs::POOL_LOG_ROUNDS;
use crate::crypto::stark::air::RATE;
use alloc::vec::Vec;

/// The first two compressions feed the third. Lane by lane: a digest is four
/// elements and binding one lane leaves the other three free.
pub(crate) fn note_edges(base: usize, span_op: usize) -> Vec<(usize, usize, usize, usize)> {
    let l = 1usize << POOL_LOG_ROUNDS;
    let root = |o: usize| base + o * span_op + l;
    let c_first = base + 2 * span_op;
    let mut sw = Vec::with_capacity(2 * RATE);
    for c in 0..RATE {
        sw.push((root(0), c, c_first, c));
        sw.push((root(1), c, c_first, RATE + c));
    }
    sw
}
