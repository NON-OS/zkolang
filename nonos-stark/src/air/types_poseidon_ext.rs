// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The proof shape of the Poseidon-committed money-grade STARK: the same DEEP
//! proof as `StarkProofExt`, but every commitment is a Poseidon Merkle root and
//! every path is Poseidon, so the whole verification is cheap to re-run inside a
//! STARK. This is the inner proof a recursive verifier folds over; the keccak
//! `StarkProofExt` is the outer proof an on-chain verifier checks.

use super::super::field::{Fp, Fp2};
use super::super::fri_poseidon_ext::FriProofExtP;
use super::poseidon::RATE;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct StarkQueryExtP {
    pub deep: Fp2,
    pub deep_path: Vec<[Fp; RATE]>,
    pub trace: Vec<Fp>,
    /// One path for the whole row: the leaf is the row's compress-chain
    /// digest, the rule `hash_periodic_row` fixes.
    pub trace_path: Vec<[Fp; RATE]>,
    pub comp: Fp2,
    pub comp_path: Vec<[Fp; RATE]>,
}

#[derive(Clone)]
pub struct StarkProofExtP {
    /// The whole trace under one root: leaf i is the compress-chain digest of
    /// row i. One absorb, one path per query, however wide the trace.
    pub trace_root: [Fp; RATE],
    pub comp_root: [Fp; RATE],
    /// The trace columns at `g^k * z` for each window row `k`, row-major in `Fp2`.
    pub ood_frame: Vec<Fp2>,
    pub fri: FriProofExtP,
    pub queries: Vec<StarkQueryExtP>,
}
