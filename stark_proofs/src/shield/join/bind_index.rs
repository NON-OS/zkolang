// NONOS Operating System (AGPL-3.0-or-later)

use super::bind::Layout;
use crate::crypto::stark::air::{IndexScalar, RATE, WIDTH};
use crate::shield::note::POOL_LOG_ROUNDS;
use crate::shield::wire_class::{pair, Class};
use alloc::vec::Vec;

/// A note is retired under the position the pool authenticated. Membership carries
/// it as path directions, the nullifier hashes it as a scalar, and a prover who can
/// move one without the other has a second nullifier for one note.
pub(crate) fn index_classes(l: &Layout) -> Vec<Class> {
    let mut c = Vec::new();
    let rounds = 1usize << POOL_LOG_ROUNDS;
    for (i, &base) in l.index.iter().enumerate() {
        // Bit zero rides the opening's first row, where the state is built from it.
        // Later bits ride the last round row of the level before the one they steer.
        c.push(pair(base, IndexScalar::BIT, l.member[i], WIDTH));
        for m in 1..l.depth {
            let dir_row = l.member[i] + (m - 1) * rounds + rounds - 1;
            c.push(pair(base + m, IndexScalar::BIT, dir_row, WIDTH));
        }
        // One level deep, so nothing injects at a slot boundary: the absorbed
        // position sits in the high half of that opening's first state.
        let state_row = l.key[i] + 3 * l.key_span[i];
        c.push(pair(base + l.depth, IndexScalar::ACC, state_row, RATE));
    }
    c
}
