// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::shield::note::POOL_LOG_ROUNDS;
use alloc::vec::Vec;

pub fn spend_pk_row(base: usize) -> usize {
    base + (1usize << POOL_LOG_ROUNDS)
}

pub fn absorbed_cm_row(base: usize, span_op: usize) -> usize {
    base + 2 * span_op
}

pub fn nullifier_edges(base: usize, span_op: usize) -> Vec<(usize, usize, usize, usize)> {
    let l = 1usize << POOL_LOG_ROUNDS;
    let first = |o: usize| base + o * span_op;
    let root = |o: usize| base + o * span_op + l;
    let mut sw = Vec::with_capacity(3 * RATE);
    for c in 0..RATE {
        sw.push((first(0), c, first(1), c));
        sw.push((root(1), c, first(2), c));
        sw.push((root(2), c, first(3), c));
    }
    sw
}
