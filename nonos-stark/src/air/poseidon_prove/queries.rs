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
use super::super::super::poseidon_merkle::{pack_base, PoseidonMerkleTree};
use super::super::poseidon::{Poseidon, RATE};
use super::super::prove_ext::{eval_base, Domain};
use super::super::types_poseidon_ext::StarkQueryExtP;
use super::trace::{CommittedTrace, TREE_CUT};
use alloc::vec::Vec;

/// One opened query: trace values by Horner from the coefficients, paths by
/// rebuilding each pruned chunk's leaves the same way. The values are exactly
/// what the dropped extension held, at exactly the positions a path needs.
pub(super) fn open(
    h: &Poseidon,
    d: &Domain,
    tr: &CommittedTrace,
    comp_d: &[Fp2],
    comp_tree: &PoseidonMerkleTree,
    deep_d: &[Fp2],
    deep_tree: &PoseidonMerkleTree,
    p: usize,
) -> StarkQueryExtP {
    let chunk = 1usize << TREE_CUT;
    let base_j = p & !(chunk - 1);
    let trace: Vec<Fp> = tr
        .coeffs
        .iter()
        .map(|cf| eval_base(cf, d.point(p)))
        .collect();
    let trace_paths: Vec<Vec<[Fp; RATE]>> = tr
        .trees
        .iter()
        .zip(&tr.coeffs)
        .map(|(tree, cf)| {
            let leaves: Vec<[Fp; RATE]> = (0..chunk)
                .map(|o| pack_base(eval_base(cf, d.point(base_j + o))))
                .collect();
            tree.open_with(h, p, &leaves)
        })
        .collect();
    StarkQueryExtP {
        deep: deep_d[p],
        deep_path: deep_tree.open(p),
        trace,
        trace_paths,
        comp: comp_d[p],
        comp_path: comp_tree.open(p),
    }
}
