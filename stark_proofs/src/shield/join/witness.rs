// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// One spent note's place in a tree the caller already has.
///
/// The circuit does not build the pool. The contract owns it, appends to it, and
/// publishes the root; a wallet reads the path out and hands it here. Tests build
/// a tree because they have to produce a witness from nothing, which is not how a
/// spend works.
pub(crate) struct Placed {
    pub siblings: Vec<[Fp; RATE]>,
    pub directions: Vec<bool>,
    /// Position in insertion order, which is what the nullifier hashes.
    pub leaf_index: usize,
}

/// The two spent notes' places, under one published root.
pub(crate) struct Places {
    pub note: [Placed; 2],
    pub root: [Fp; RATE],
}

impl Placed {
    pub fn depth(&self) -> usize {
        self.siblings.len()
    }
}
