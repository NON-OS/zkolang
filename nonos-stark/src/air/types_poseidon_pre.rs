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

//! The poseidon-transcript proof with the periodic sidecar. Same shape as the
//! keccak preprocessed form: the claimed periodic values at z ride the proof,
//! and each query opens the committed periodic row, so a verifier holds the
//! periodic root as a constant instead of recomputing the schedule. For the
//! recursion that constant is what deletes the schedule-recompute region, which
//! was half the outer circuit's rows.

use super::super::field::{Fp, Fp2};
use super::poseidon::RATE;
use super::types_poseidon_ext::StarkProofExtP;
use alloc::vec::Vec;

/// One opened periodic row: the values at the queried position and the path
/// to the baked root.
#[derive(Clone)]
pub struct PeriodicOpeningP {
    pub row: Vec<Fp>,
    pub path: Vec<[Fp; RATE]>,
}

/// A poseidon-transcript proof with the periodic sidecar. `openings` parallels
/// the proof's consistency queries in order.
#[derive(Clone)]
pub struct StarkProofExtPPre {
    pub proof: StarkProofExtP,
    /// The claimed periodic-column evaluations at the out-of-domain point.
    pub periodic_z: Vec<Fp2>,
    pub openings: Vec<PeriodicOpeningP>,
}
