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

use super::super::super::field::Fp;
use super::super::super::poly::{intt, lde_from_coeffs};
use super::super::super::poseidon_merkle::PrunedPoseidonTree;
use super::super::periodic_poseidon::hash_periodic_row;
use super::super::poseidon::{Poseidon, RATE};
use super::super::prove_ext::{eval_base, Domain};
use alloc::vec::Vec;

/// Levels dropped from the trace tree; a query rebuilds its own chunk.
pub(crate) const TREE_CUT: u32 = 6;

/// The whole trace under one root: leaf i is the compress-chain digest of
/// row i, the same rule the periodic commitment uses, so the recursion binds
/// an opened row with the chain-plus-path opening it already knows. One tree
/// instead of one per column: seventeen openings per query become four, and
/// the transcript absorbs one root instead of fourteen.
pub(crate) struct WideTrace {
    pub coeffs: Vec<Vec<Fp>>,
    pub tree: PrunedPoseidonTree,
}

pub(crate) fn commit_wide(h: &Poseidon, d: &Domain, trace: &[Fp]) -> WideTrace {
    let coeffs: Vec<Vec<Fp>> = crate::par::map_index(d.width, |c| {
        let column: Vec<Fp> = (0..d.t).map(|i| trace[i * d.width + c]).collect();
        intt(&column, d.g)
    });
    let columns_d: Vec<Vec<Fp>> =
        crate::par::map_slice(&coeffs, |cf| lde_from_coeffs(cf, d.shift, d.omega, d.n));
    let leaves: Vec<[Fp; RATE]> = crate::par::map_index(d.n, |i| {
        let row: Vec<Fp> = columns_d.iter().map(|col| col[i]).collect();
        hash_periodic_row(h, &row)
    });
    let tree = PrunedPoseidonTree::commit(h, &leaves, TREE_CUT);
    WideTrace { coeffs, tree }
}

/// Row j of the extension by Horner from the coefficients: the values the
/// dropped extension held, which is what a pruned chunk's leaves rebuild from.
pub(crate) fn row_at(d: &Domain, coeffs: &[Vec<Fp>], j: usize) -> Vec<Fp> {
    let x = d.point(j);
    coeffs.iter().map(|cf| eval_base(cf, x)).collect()
}
