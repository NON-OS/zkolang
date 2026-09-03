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

//! The Poseidon-committed money-grade DEEP STARK prover: the same construction as
//! `prove_ext`, but the trace, composition, and DEEP polynomial are committed with
//! Poseidon Merkle trees and the transcript is the Poseidon sponge, and the
//! low-degree test is the Poseidon money-grade FRI. Every step is then cheap to
//! re-verify inside a STARK, which is what makes a proof from here recursable.

use super::super::air::{Poseidon, RATE};
use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_poseidon_ext::fri_prove_poseidon_ext;
use super::super::poly::eval_cols_on_subgroup_ext;
use super::super::poseidon_merkle::{pack_ext, PoseidonMerkleTree};
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::spec::AirExt;
use super::types_poseidon_ext::{StarkProofExtP, StarkQueryExtP};
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Prove `trace` satisfies `air` at money-grade soundness, committed with Poseidon.
/// `extra_blowup_bits` sets the FRI rate exactly as the keccak prover.
pub fn stark_prove_poseidon_ext<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
) -> StarkProofExtP {
    stark_prove_poseidon_ext_pub(
        air,
        trace,
        n_queries,
        grind_bits,
        extra_blowup_bits,
        hasher,
        &[],
    )
}

/// The same prover, seeding the transcript with `publics` before the trace roots so
/// the proof is bound to those public inputs by Fiat-Shamir. A recursive verifier
/// replays the same seed, exposing the publics in its transcript column.
#[allow(clippy::too_many_arguments)]
pub fn stark_prove_poseidon_ext_pub<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> StarkProofExtP {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();

    let g = root_of_unity(log_t);
    let shift = Fp::from_u64(SHIFT);

    let mut transcript = PoseidonTranscript::new(hasher.clone());
    for &p in publics {
        transcript.absorb(p);
    }
    // The whole trace under one root: the extension is transient, hashed row
    // by row into a pruned tree and dropped, and the transcript absorbs one
    // digest however wide the trace is.
    let d = super::prove_ext::Domain::of(air, extra_blowup_bits);
    let wt = super::poseidon_prove::commit_wide(hasher, &d, trace);
    let trace_coeffs = &wt.coeffs;
    transcript.absorb_digest(&wt.tree.root());

    let coeffs: Vec<Fp2> = (0..num_coeffs(air))
        .map(|_| transcript.challenge_fp2())
        .collect();

    let periodic_cols = air.periodic_columns();
    // Composition and DEEP run the shared streamed passes: the trace exists as
    // coefficients, each pass extends one coset at a time, and the arithmetic
    // is the keccak path's to the element. Only the transcript and the trees
    // differ between the two provers now.
    let pc = super::prove_ext::periodic_coeffs(&periodic_cols, &d);
    let comp_d = super::prove_ext::over_domain(air, &d, trace_coeffs, &pc, &coeffs);
    let comp_leaves: Vec<[Fp; RATE]> = comp_d.iter().map(|v| pack_ext(*v)).collect();
    let comp_tree = PoseidonMerkleTree::commit(hasher, &comp_leaves);
    transcript.absorb_digest(&comp_tree.root());

    let z = draw_ood_point_poseidon(&mut transcript, shift, n, t);
    let ood_frame = super::prove_ext::ood_frame(trace_coeffs, &d, z);
    for value in &ood_frame {
        transcript.absorb(value.c0);
        transcript.absorb(value.c1);
    }

    let periodic_z: Vec<Fp2> = eval_cols_on_subgroup_ext(g, t, &periodic_cols, z);
    let comp_z = compose_ext(air, g, z, &ood_frame, &periodic_z, &coeffs);

    let deep_coeffs: Vec<Fp2> = (0..width * window_size + 1)
        .map(|_| transcript.challenge_fp2())
        .collect();

    let deep_d = super::prove_ext::deep_over_domain(
        &d,
        trace_coeffs,
        &comp_d,
        &ood_frame,
        comp_z,
        z,
        &deep_coeffs,
    );

    let fri = fri_prove_poseidon_ext(
        &deep_d,
        shift,
        fri_log_blowup,
        n_queries,
        grind_bits,
        hasher,
    );
    let deep_leaves: Vec<[Fp; RATE]> = deep_d.iter().map(|v| pack_ext(*v)).collect();
    let deep_tree = PoseidonMerkleTree::commit(hasher, &deep_leaves);
    transcript.absorb_digest(&fri.roots[0]);

    let mut queries: Vec<StarkQueryExtP> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(n);
        queries.push(super::poseidon_prove::open_query(
            hasher, &d, &wt, &comp_d, &comp_tree, &deep_d, &deep_tree, p,
        ));
    }

    StarkProofExtP {
        trace_root: wt.tree.root(),
        comp_root: comp_tree.root(),
        ood_frame,
        fri,
        queries,
    }
}
