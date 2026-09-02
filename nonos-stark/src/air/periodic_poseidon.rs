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

//! The poseidon periodic commitment: the schedule's evaluation over the whole
//! domain, committed once, opened per query. A verifier that holds this root
//! never recomputes the schedule, which for the recursion deletes the region
//! that was half its rows. Registration and proving flow through the same
//! helper, so the root a verifier bakes and the root a prover commits are one
//! object by construction, the same discipline as the keccak path.

use super::super::field::Fp;
use super::super::poseidon_merkle::PrunedPoseidonTree;
use super::poseidon::{Poseidon, RATE};
use super::prove_ext::{extend, periodic_coeffs, Domain};
use super::spec::AirExt;
use alloc::vec::Vec;

/// Bottom levels dropped from the committed tree; queries rebuild their chunk.
pub(crate) const PERIODIC_TREE_CUT: u32 = 6;

/// One periodic row folded to a digest: a compress chain from the zero
/// digest, one chunk per step. The rule is pure compression on purpose. The
/// recursion authenticates these digests with the same membership region that
/// walks Merkle paths, and a compress chain from a known start is exactly an
/// opening whose siblings are the chunks, so the row binds in-circuit with
/// machinery that already exists.
pub fn hash_periodic_row(h: &Poseidon, row: &[Fp]) -> [Fp; RATE] {
    let mut d = [Fp::ZERO; RATE];
    let mut i = 0usize;
    while i < row.len() {
        let mut chunk = [Fp::ZERO; RATE];
        let take = (row.len() - i).min(RATE);
        chunk[..take].copy_from_slice(&row[i..i + take]);
        d = h.compress(&d, &chunk);
        i += RATE;
    }
    d
}

/// The periodic coefficients and the pruned tree over their coset extension.
/// The extension is never held: leaves hash one coset at a time.
pub fn periodic_tree_poseidon<A: AirExt>(
    air: &A,
    extra_blowup_bits: u32,
    h: &Poseidon,
) -> (Vec<Vec<Fp>>, PrunedPoseidonTree) {
    let d = Domain::of(air, extra_blowup_bits);
    let cols = air.periodic_columns();
    let coeffs = periodic_coeffs(&cols, &d);
    if cols.is_empty() {
        return (
            coeffs,
            PrunedPoseidonTree::commit(h, &[], PERIODIC_TREE_CUT),
        );
    }
    let mut leaves = alloc::vec![[Fp::ZERO; RATE]; d.n];
    for c in 0..d.blowup {
        let per = extend(&coeffs, &d, c);
        let hashed: Vec<[Fp; RATE]> = crate::par::map_index(d.t, |i| {
            let row: Vec<Fp> = per.iter().map(|col| col[i]).collect();
            hash_periodic_row(h, &row)
        });
        for (i, leaf) in hashed.into_iter().enumerate() {
            leaves[c + d.blowup * i] = leaf;
        }
    }
    let tree = PrunedPoseidonTree::commit(h, &leaves, PERIODIC_TREE_CUT);
    (coeffs, tree)
}

/// The baked root a verifier key binds for the poseidon path.
pub fn periodic_root_poseidon<A: AirExt>(
    air: &A,
    extra_blowup_bits: u32,
    h: &Poseidon,
) -> [Fp; RATE] {
    periodic_tree_poseidon(air, extra_blowup_bits, h).1.root()
}
