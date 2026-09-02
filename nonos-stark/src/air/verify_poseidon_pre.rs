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

//! The preprocessed poseidon verifier: the periodic root arrives as a baked
//! constant, the proof carries the claimed periodic values at z and one
//! opened periodic row per query, and DEEP holds one quotient per periodic
//! column against the claims. Nothing here recomputes the schedule, which is
//! the whole point: a recursion that replays this algorithm carries the root
//! as a constant instead of half its rows.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::fri_poseidon_ext::fri_verify_poseidon_ext;
use super::super::poseidon_merkle::{pack_base, pack_ext, verify_path};
use super::composition::{compose_ext, domain_params_blown};
use super::periodic_poseidon::hash_periodic_row;
use super::poseidon::{Poseidon, RATE};
use super::spec::AirExt;
use super::types_poseidon_pre::StarkProofExtPPre;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Verify a sidecar proof against the baked `periodic_root`, bound to
/// `publics`. Exact transcript counterpart of `stark_prove_poseidon_pre_pub`.
#[allow(clippy::too_many_arguments)]
pub fn stark_verify_poseidon_pre_pub<A: AirExt>(
    air: &A,
    pre: &StarkProofExtPPre,
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
    periodic_root: &[Fp; RATE],
) -> bool {
    let proof = &pre.proof;
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, fri_log_blowup) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();
    let n_periodic = air.periodic_columns().len();

    if proof.trace_roots.len() != width
        || proof.ood_frame.len() != window_size * width
        || proof.queries.len() != n_queries
        || pre.periodic_z.len() != n_periodic
        || pre.openings.len() != n_queries
    {
        return false;
    }

    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    let mut r =
        super::replay::replay(air, proof, Some(&pre.periodic_z), extra_blowup_bits, hasher, publics);
    let (coeffs, z, deep_coeffs) = (r.coeffs.clone(), r.z, r.deep_coeffs.clone());
    let comp_z = compose_ext(air, g, z, &proof.ood_frame, &pre.periodic_z, &coeffs);

    if !fri_verify_poseidon_ext(
        &proof.fri,
        shift,
        log_n,
        fri_log_blowup,
        n_queries,
        grind_bits,
        hasher,
    ) {
        return false;
    }
    let deep_root = proof.fri.roots[0];

    for (qd, po) in proof.queries.iter().zip(&pre.openings) {
        let p = r.ts.challenge_index(n);
        if qd.trace.len() != width || qd.trace_paths.len() != width || po.row.len() != n_periodic {
            return false;
        }
        if !verify_path(hasher, &deep_root, p, pack_ext(qd.deep), &qd.deep_path)
            || !verify_path(
                hasher,
                &proof.comp_root,
                p,
                pack_ext(qd.comp),
                &qd.comp_path,
            )
            || !verify_path(
                hasher,
                periodic_root,
                p,
                hash_periodic_row(hasher, &po.row),
                &po.path,
            )
        {
            return false;
        }
        for c in 0..width {
            if !verify_path(
                hasher,
                &proof.trace_roots[c],
                p,
                pack_base(qd.trace[c]),
                &qd.trace_paths[c],
            ) {
                return false;
            }
        }

        let x = shift * omega.pow(p as u64);
        let xe = Fp2::from_base(x);
        let mut acc = Fp2::ZERO;
        for k in 0..window_size {
            let zk = z * Fp2::from_base(g.pow(k as u64));
            let inv_x_zk = (xe - zk).inv();
            for c in 0..width {
                let claimed = proof.ood_frame[k * width + c];
                acc = acc
                    + deep_coeffs[k * width + c]
                        * ((Fp2::from_base(qd.trace[c]) - claimed) * inv_x_zk);
            }
        }
        let inv_x_z = (xe - z).inv();
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((qd.comp - comp_z) * inv_x_z);
        for (pi, &pv) in po.row.iter().enumerate() {
            let pcoeff = deep_coeffs[width * window_size + 1 + pi];
            acc = acc + pcoeff * ((Fp2::from_base(pv) - pre.periodic_z[pi]) * inv_x_z);
        }
        if acc != qd.deep {
            return false;
        }
    }

    true
}
