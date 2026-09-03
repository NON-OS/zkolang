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

use super::super::super::field::{Fp, Fp2};
use super::super::super::poseidon_merkle::PoseidonMerkleTree;
use super::super::periodic_poseidon::hash_periodic_row;
use super::super::poseidon::{Poseidon, RATE};
use super::super::prove_ext::Domain;
use super::super::types_poseidon_ext::StarkQueryExtP;
use super::trace::{row_at, WideTrace, TREE_CUT};
use alloc::vec::Vec;

/// One opened query: the row's values by Horner from the coefficients, one
/// path whose pruned chunk is rebuilt by hashing each neighbouring row the
/// same way the commit did. The leaf binds all the columns at once.
#[allow(clippy::too_many_arguments)]
pub(crate) fn open(
    h: &Poseidon,
    d: &Domain,
    wt: &WideTrace,
    comp_d: &[Fp2],
    comp_tree: &PoseidonMerkleTree,
    deep_d: &[Fp2],
    deep_tree: &PoseidonMerkleTree,
    p: usize,
) -> StarkQueryExtP {
    let chunk = 1usize << TREE_CUT;
    let base_j = p & !(chunk - 1);
    let leaves: Vec<[Fp; RATE]> = (0..chunk)
        .map(|o| hash_periodic_row(h, &row_at(d, &wt.coeffs, base_j + o)))
        .collect();
    StarkQueryExtP {
        deep: deep_d[p],
        deep_path: deep_tree.open(p),
        trace: row_at(d, &wt.coeffs, p),
        trace_path: wt.tree.open_with(h, p, &leaves),
        comp: comp_d[p],
        comp_path: comp_tree.open(p),
    }
}
