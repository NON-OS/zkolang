// NONOS Operating System (AGPL-3.0-or-later)
//! The nullifier set as an indexed Merkle tree, per spec/nullifier-imt.md.
//!
//! Membership of the note pool is an append-only tree; a nullifier set needs
//! non-membership, which a sparse tree answers with a 256 deep empty-leaf proof
//! and this answers with one short path to the leaf below the key.

mod fold;
pub mod hash;
mod insert;
mod last;
mod leaf;
mod merge;
mod order;
mod set;
#[cfg(test)]
mod test;
mod witnessed;

pub use fold::{refold, Tree};
pub use insert::{chain, writes_are_distinct, Low, Step};
pub use last::last_is_the_maximum;
pub use leaf::{Leaf, IMT_LEAF_DOMAIN, IMT_LEAF_LIMBS};
pub use merge::{same, stitch, Range, State};
pub use order::{cmp, excludes};
pub use set::Set;
pub use witnessed::{root_of, Path};
