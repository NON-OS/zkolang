// NONOS Operating System (AGPL-3.0-or-later)
//! The nullifier set as an indexed Merkle tree, per spec/nullifier-imt.md.
//!
//! Membership of the note pool is an append-only tree; a nullifier set needs
//! non-membership, which a sparse tree answers with a 256 deep empty-leaf proof
//! and this answers with one short path to the leaf below the key.

mod fold;
pub(crate) mod hash;
mod insert;
mod last;
mod leaf;
mod merge;
mod order;
mod set;
mod witnessed;
#[cfg(test)]
mod test;

pub(crate) use fold::{refold, Tree};
pub(crate) use insert::{chain, writes_are_distinct, Low, Step};
pub(crate) use last::last_is_the_maximum;
pub(crate) use leaf::{Leaf, IMT_LEAF_DOMAIN, IMT_LEAF_LIMBS};
pub(crate) use merge::{same, stitch, Range, State};
pub(crate) use witnessed::{root_of, Path};
pub(crate) use set::Set;
pub(crate) use order::{cmp, excludes};
