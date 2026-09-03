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
use super::super::super::poly::{intt, lde, lde_from_coeffs};
use super::super::super::poseidon_merkle::{pack_base, PrunedPoseidonTree};
use super::super::periodic_poseidon::hash_periodic_row;
use super::super::poseidon::{Poseidon, RATE};
use super::super::prove_ext::Domain;
use alloc::vec::Vec;

/// Levels dropped from each per-column tree; a query rebuilds its own chunk.
pub(super) const TREE_CUT: u32 = 6;

/// Every trace column committed: coefficients kept, the extension hashed into
/// a pruned tree and dropped. Columns are independent and run together; the
/// caller absorbs the roots in column order, which is what a verifier replays.
pub(super) struct CommittedTrace {
    pub coeffs: Vec<Vec<Fp>>,
    pub trees: Vec<PrunedPoseidonTree>,
    pub roots: Vec<[Fp; RATE]>,
}

pub(super) fn commit(h: &Poseidon, d: &Domain, trace: &[Fp]) -> CommittedTrace {
    let built: Vec<(PrunedPoseidonTree, Vec<Fp>)> = crate::par::map_index(d.width, |c| {
        let column: Vec<Fp> = (0..d.t).map(|i| trace[i * d.width + c]).collect();
        let column_d = lde(&column, d.g, d.shift, d.omega, d.n);
        let leaves: Vec<[Fp; RATE]> = column_d.iter().map(|v| pack_base(*v)).collect();
        let tree = PrunedPoseidonTree::commit(h, &leaves, TREE_CUT);
        (tree, intt(&column, d.g))
    });
    let mut out = CommittedTrace {
        coeffs: Vec::with_capacity(d.width),
        trees: Vec::with_capacity(d.width),
        roots: Vec::with_capacity(d.width),
    };
    for (tree, coeffs) in built {
        out.roots.push(tree.root());
        out.coeffs.push(coeffs);
        out.trees.push(tree);
    }
    out
}

/// The whole trace under one root: leaf i is the compress-chain digest of
/// row i, the same rule the periodic commitment uses, so the recursion binds
/// an opened row with the chain-plus-path opening it already knows. One tree
/// instead of one per column: seventeen openings per query become four, and
/// the transcript absorbs one root instead of fourteen.
pub(super) struct WideTrace {
    pub coeffs: Vec<Vec<Fp>>,
    pub tree: PrunedPoseidonTree,
}

pub(super) fn commit_wide(h: &Poseidon, d: &Domain, trace: &[Fp]) -> WideTrace {
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
