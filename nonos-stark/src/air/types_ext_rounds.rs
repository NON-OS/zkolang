// NONOS Operating System (AGPL-3.0-or-later)
//! A preprocessed proof whose trace was committed in two rounds.
//!
//! The region columns are committed first and the permutation columns second,
//! with the permutation challenges drawn in between. So the trace has two
//! roots and a query's row has two paths, one per half.
//!
//! `proof.trace_root` carries the region half's root, and the row in
//! `proof.queries[i].trace` is still the whole row: regions below
//! `region_width`, permutation columns from there up. The verifier splits it
//! and checks each half against its own root.

use super::types_ext_pre::StarkProofExtPre;
use alloc::vec::Vec;

pub struct StarkProofExtRounds {
    pub pre: StarkProofExtPre,
    /// Root of the second round, over the permutation columns alone.
    pub perm_root: [u8; 32],
    /// Where the row splits. Emitted rather than assumed, because a verifier
    /// that split it anywhere else would be checking two roots against the
    /// wrong halves and would still see two valid Merkle walks.
    pub region_width: usize,
    /// Per query, in the same order as `pre.proof.queries`: the path
    /// authenticating the permutation half of that row under `perm_root`.
    pub perm_paths: Vec<Vec<[u8; 32]>>,
}
