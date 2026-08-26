// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;

/// What settleAggregate swaps: the note tree, where the next leaf lands, and the
/// nullifier set. The root proof exposes the pair it moved between, and the
/// contract requires the old half equals what it currently holds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Carried {
    pub note_root: [Fp; RATE],
    pub next_index: u64,
    pub nullifier_root: [Fp; RATE],
}

/// One node's claim: it took the chain from `old` to `new`.
#[derive(Clone, Copy)]
pub struct Node {
    pub old: Carried,
    pub new: Carried,
}
