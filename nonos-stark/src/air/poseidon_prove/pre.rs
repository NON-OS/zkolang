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

use super::super::super::field::Fp2;
use super::super::super::fri_poseidon_ext::fri_prove_poseidon_ext;
use super::super::super::poseidon_merkle::{pack_ext, PoseidonMerkleTree};
use super::super::super::poseidon_transcript::PoseidonTranscript;
use super::super::composition::num_coeffs;
use super::super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::super::periodic_poseidon::periodic_tree_poseidon;
use super::super::poseidon::{Poseidon, RATE};
use super::super::prove_ext::{comp_at_z, ood_frame, over_domain, periodic_at_z, Domain};
use super::super::prove_ext_pre::pre_deep_over_domain;
use super::super::spec::AirExt;
use super::super::types_poseidon_ext::StarkProofExtP;
use super::super::types_poseidon_pre::{PeriodicOpeningP, StarkProofExtPPre};
use super::{queries, sidecar, trace};
use crate::field::Fp;
use alloc::vec::Vec;

/// Prove `trace` with the periodic sidecar, bound to `publics`. The verifier
/// holds the matching baked periodic root; the periodic tree here comes
/// through the registration helper, so the two are one object by
/// construction. The transcript mirrors the plain poseidon prover until the
/// frame, then absorbs the periodic claims, and DEEP carries one quotient per
/// periodic column against them.
#[allow(clippy::too_many_arguments)]
pub fn stark_prove_poseidon_pre_pub<A: AirExt>(
    air: &A,
    witness: &[Fp],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    h: &Poseidon,
    publics: &[Fp],
) -> StarkProofExtPPre {
    let d = Domain::of(air, extra_blowup_bits);

    let mut transcript = PoseidonTranscript::new(h.clone());
    for &p in publics {
        transcript.absorb(p);
    }
    let tr = trace::commit(h, &d, witness);
    for root in &tr.roots {
        transcript.absorb_digest(root);
    }

    let coeffs: Vec<Fp2> = (0..num_coeffs(air))
        .map(|_| transcript.challenge_fp2())
        .collect();

    let periodic_cols = air.periodic_columns();
    let (pc, p_tree) = periodic_tree_poseidon(air, extra_blowup_bits, h);
    let comp_d = over_domain(air, &d, &tr.coeffs, &pc, &coeffs);
    let comp_leaves: Vec<[Fp; RATE]> = comp_d.iter().map(|v| pack_ext(*v)).collect();
    let comp_tree = PoseidonMerkleTree::commit(h, &comp_leaves);
    transcript.absorb_digest(&comp_tree.root());

    let z = draw_ood_point_poseidon(&mut transcript, d.shift, d.n, d.t);
    let frame = ood_frame(&tr.coeffs, &d, z);
    for value in &frame {
        transcript.absorb(value.c0);
        transcript.absorb(value.c1);
    }
    let periodic_z = periodic_at_z(&d, &periodic_cols, z);
    for value in &periodic_z {
        transcript.absorb(value.c0);
        transcript.absorb(value.c1);
    }
    let comp_z = comp_at_z(air, &d, &frame, &periodic_z, z, &coeffs);

    let deep_coeffs: Vec<Fp2> = (0..d.width * d.window + 1 + periodic_cols.len())
        .map(|_| transcript.challenge_fp2())
        .collect();
    let deep_d = pre_deep_over_domain(
        &d,
        &tr.coeffs,
        &pc,
        &comp_d,
        &frame,
        &periodic_z,
        comp_z,
        z,
        &deep_coeffs,
    );

    let fri = fri_prove_poseidon_ext(&deep_d, d.shift, d.fri_log_blowup, n_queries, grind_bits, h);
    let deep_leaves: Vec<[Fp; RATE]> = deep_d.iter().map(|v| pack_ext(*v)).collect();
    let deep_tree = PoseidonMerkleTree::commit(h, &deep_leaves);
    transcript.absorb_digest(&fri.roots[0]);

    let mut qs = Vec::with_capacity(n_queries);
    let mut openings: Vec<PeriodicOpeningP> = Vec::with_capacity(n_queries);
    for _ in 0..n_queries {
        let p = transcript.challenge_index(d.n);
        qs.push(queries::open(
            h, &d, &tr, &comp_d, &comp_tree, &deep_d, &deep_tree, p,
        ));
        openings.push(sidecar::open(h, &d, &pc, &p_tree, p));
    }

    StarkProofExtPPre {
        proof: StarkProofExtP {
            trace_roots: tr.roots,
            comp_root: comp_tree.root(),
            ood_frame: frame,
            fri,
            queries: qs,
        },
        periodic_z,
        openings,
    }
}
