// NONOS Operating System (AGPL-3.0-or-later)
//! The nullifier set as an indexed Merkle tree, per spec/nullifier-imt.md.
//!
//! Membership of the note pool is an append-only tree; a nullifier set needs
//! non-membership, which a sparse tree answers with a 256 deep empty-leaf proof
//! and this answers with one short path to the leaf below the key.

mod hash;
mod leaf;
mod order;
#[cfg(test)]
mod test;

pub(crate) use leaf::{Leaf, IMT_LEAF_DOMAIN, IMT_LEAF_LIMBS};
pub(crate) use order::{cmp, excludes};
