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

//! Opening the periodic sidecar for a query. The preprocessed path holds the periodic schedule
//! as a baked commitment rather than recomputing it, so a proof carries the claimed periodic
//! values at the out-of-domain point and, per query, one opened row of the committed schedule
//! with its path to the baked root. This is the pass that emits that opened row, the object a
//! recursion binds against the root it holds as a constant instead of half its rows.

use super::super::super::field::Fp;
use super::super::super::poseidon_merkle::PrunedPoseidonTree;
use super::super::periodic_poseidon::{hash_periodic_row, PERIODIC_TREE_CUT};
use super::super::poseidon::{Poseidon, RATE};
use super::super::prove_ext::{eval_base, Domain};
use super::super::types_poseidon_pre::PeriodicOpeningP;
use alloc::vec::Vec;

/// The periodic sidecar for one query: the committed row's values and its path
/// to the baked root, both rebuilt from coefficients. Same values the
/// committed extension held, same path the unpruned tree would return.
pub(super) fn open(
    h: &Poseidon,
    d: &Domain,
    pc: &[Vec<Fp>],
    tree: &PrunedPoseidonTree,
    p: usize,
) -> PeriodicOpeningP {
    let row: Vec<Fp> = pc.iter().map(|cf| eval_base(cf, d.point(p))).collect();
    let chunk = 1usize << PERIODIC_TREE_CUT;
    let base = p & !(chunk - 1);
    let leaves: Vec<[Fp; RATE]> = (0..chunk)
        .map(|o| {
            let r: Vec<Fp> = pc
                .iter()
                .map(|cf| eval_base(cf, d.point(base + o)))
                .collect();
            hash_periodic_row(h, &r)
        })
        .collect();
    PeriodicOpeningP {
        row,
        path: tree.open_with(h, p, &leaves),
    }
}
