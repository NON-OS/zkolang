// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::{IndexScalar, RATE, WIDTH};
use crate::shield::note::POOL_LOG_ROUNDS;
use crate::shield::wire_class::{pair, Class};
use alloc::vec::Vec;

/// The position a note is retired under is the position the pool authenticated.
///
/// Membership carries the position as path directions; the nullifier hashes it as
/// a scalar. Left apart, a prover keeps the note and the ownership honest and
/// moves only the scalar, which mints a second nullifier for one note. So the
/// recovery chain's bits are pinned to the directions, and its result to the cell
/// the fourth compression absorbs.
pub fn index_classes(l: &Layout) -> Vec<Class> {
    let mut c = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    for (i, &base) in l.index.iter().enumerate() {
        // Level m's direction rides the membership trace on the last round row of
        // level m-1: the trace carries directions[1..depth], because directions[0]
        // was consumed building the opening's initial state and lives only in which
        // half of that state the leaf occupies. Bit zero is not bound here.
        for m in 1..l.depth {
            let dir_row = l.member[i] + (m - 1) * rounds + rounds - 1;
            c.push(pair(base + m, IndexScalar::BIT, dir_row, WIDTH));
        }
        // The fourth compression absorbs the position as its sibling. Its opening is
        // one level deep, so no slot boundary ever injects: the sibling sits in the
        // high half of that opening's first state, which is where the position is.
        let state_row = l.key[i] + 3 * l.key_span[i];
        c.push(pair(base + l.depth, IndexScalar::ACC, state_row, RATE));
    }
    c
}
