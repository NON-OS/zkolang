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

//! The DEEP terms of a preprocessed poseidon proof's query, for a recursion
//! that carries the periodic root as a constant. The transcript walk comes
//! from `replay`; what lives here is only the term list: the frame quotients,
//! the composition, and one quotient per periodic column valued at the opened
//! row, so the recursion's deep identity reads the cells its auth walks.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::composition::{compose_ext, domain_params_blown};
use super::deep_check_ext::DeepTerm;
use super::poseidon::Poseidon;
use super::replay::{query_index, replay};
use super::spec::AirExt;
use super::types_poseidon_ext::StarkProofExtP;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// The terms of query `k`, its evaluation point, and the query's DEEP value.
#[allow(clippy::too_many_arguments)]
pub fn deep_terms_pre_queryk<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    periodic_z: &[Fp2],
    opened_row: &[Fp],
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
    query: usize,
) -> (Vec<DeepTerm>, Fp2, Fp2) {
    let log_t = air.log_trace_len();
    let width = air.trace_width();
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();
    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);

    let mut r = replay(air, proof, Some(periodic_z), extra_blowup_bits, hasher, publics);
    let comp_z = compose_ext(air, g, r.z, &proof.ood_frame, periodic_z, &r.coeffs);
    let p = query_index(&mut r, n, query);

    let qd = &proof.queries[query];
    let x = Fp2::from_base(Fp::from_u64(SHIFT) * omega.pow(p as u64));

    let mut terms: Vec<DeepTerm> =
        Vec::with_capacity(width * window_size + 1 + periodic_z.len());
    for k in 0..window_size {
        let zk = r.z * Fp2::from_base(g.pow(k as u64));
        for c in 0..width {
            terms.push(DeepTerm {
                val: Fp2::from_base(qd.trace[c]),
                claim: proof.ood_frame[k * width + c],
                point: zk,
                coeff: r.deep_coeffs[k * width + c],
            });
        }
    }
    terms.push(DeepTerm {
        val: qd.comp,
        claim: comp_z,
        point: r.z,
        coeff: r.deep_coeffs[width * window_size],
    });
    for (pi, &pv) in opened_row.iter().enumerate() {
        terms.push(DeepTerm {
            val: Fp2::from_base(pv),
            claim: periodic_z[pi],
            point: r.z,
            coeff: r.deep_coeffs[width * window_size + 1 + pi],
        });
    }

    (terms, x, qd.deep)
}
