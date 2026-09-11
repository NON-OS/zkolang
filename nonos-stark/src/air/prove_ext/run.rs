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

//! The prover driver: the transcript walk in one place, so the order a prover commits and
//! draws is written once and every stage hangs off it. It commits the trace, draws the
//! composition coefficients, builds and commits the composition, draws the out-of-domain
//! point, absorbs the frame, draws the DEEP coefficients, builds and commits the DEEP
//! polynomial, runs FRI, and opens the queries. Nothing here holds an algorithm; each step
//! calls its stage module, so the driver reads as the protocol it is.

use super::super::super::field::Fp2;
use super::super::super::fri_ext::fri_prove_ext;
use super::super::super::merkle::MerkleTree;
use super::super::super::transcript::Transcript;
use super::super::composition::num_coeffs;
use super::super::spec::AirExt;
use super::super::types_ext::StarkProofExt;
use super::ood::draw_ood_point_ext;
use super::setup::Domain;
use super::{commit, compose, coset, deep, frame, queries};
use crate::field::Fp;
use alloc::vec::Vec;

/// The prover body. The transcript order is the protocol and every absorb and
/// draw below matches the materialized prover this replaced, so the two emit
/// identical bytes; what changed is that no column is ever held over the full
/// evaluation domain. The trace lives as coefficients and each pass extends
/// one coset at a time, which is what lets the largest instances prove in the
/// memory of a laptop instead of a server.
pub(super) fn prove<A: AirExt>(
    air: &A,
    trace: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    context: &[u8],
    blind: &[Vec<Fp>],
) -> StarkProofExt {
    let d = Domain::of(air, extra_blowup_bits);

    let mut transcript = Transcript::new(b"NONOS-STARK-EXT");
    if !context.is_empty() {
        transcript.absorb_digest(&crate::hash::keccak256(context));
    }

    let trace_coeffs = coset::trace_coeffs(trace, &d);
    // Zero-knowledge blinding: replace each column f by f + r * Z_H. On the trace
    // domain Z_H is zero, so the values and every constraint are unchanged; off it,
    // where the queries open, the values are randomized by the prover's secret r,
    // so the openings carry nothing about the witness. Everything downstream, the
    // commitment, the composition, the frame, and the DEEP, reads the blinded
    // coefficients, so the whole proof is consistent and the verifier needs no
    // knowledge of r. An empty `blind` is the plain, non-hiding prover.
    let trace_coeffs = if blind.is_empty() {
        trace_coeffs
    } else {
        assert_eq!(blind.len(), d.width, "one blinding polynomial per trace column");
        trace_coeffs
            .into_iter()
            .zip(blind)
            .map(|(cf, r)| crate::poly::blind_coeffs(&cf, d.t, r))
            .collect()
    };
    let trace_tree = commit::wide_streamed(&trace_coeffs, &d);
    let trace_root = trace_tree.root();
    transcript.absorb_digest(&trace_root);

    let coeffs: Vec<Fp2> = (0..num_coeffs(air))
        .map(|_| transcript.challenge_fp2())
        .collect();

    let periodic_cols = air.periodic_columns();
    let periodic_coeffs = coset::periodic_coeffs(&periodic_cols, &d);
    let comp_d = compose::over_domain(air, &d, &trace_coeffs, &periodic_coeffs, &coeffs);
    let comp_tree = MerkleTree::commit_ext(&comp_d);
    transcript.absorb_digest(&comp_tree.root());

    let z = draw_ood_point_ext(&mut transcript, d.shift, d.n, d.t);
    let ood_frame = frame::ood_frame(&trace_coeffs, &d, z);
    for value in &ood_frame {
        transcript.absorb_fp(value.c0);
        transcript.absorb_fp(value.c1);
    }
    let periodic_z = frame::periodic_at_z(&d, &periodic_cols, z);
    let comp_z = frame::comp_at_z(air, &d, &ood_frame, &periodic_z, z, &coeffs);

    let deep_coeffs: Vec<Fp2> = (0..d.width * d.window + 1)
        .map(|_| transcript.challenge_fp2())
        .collect();
    let deep_d = deep::over_domain(
        &d,
        &trace_coeffs,
        &comp_d,
        &ood_frame,
        comp_z,
        z,
        &deep_coeffs,
    );

    let fri = fri_prove_ext(&deep_d, d.shift, d.fri_log_blowup, n_queries, grind_bits);
    let deep_tree = MerkleTree::commit_ext(&deep_d);
    transcript.absorb_digest(&fri.roots[0]);

    let queries = queries::open(
        &mut transcript,
        n_queries,
        &d,
        &trace_coeffs,
        &trace_tree,
        &comp_d,
        &comp_tree,
        &deep_d,
        &deep_tree,
    );
    StarkProofExt {
        trace_root,
        comp_root: comp_tree.root(),
        ood_frame,
        fri,
        queries,
    }
}
