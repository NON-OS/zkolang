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

//! The keccak transcript replay, written once: the challenge sequence of
//! `stark_verify_ext`, yielded as values instead of consumed inline, for any
//! consumer that must rederive what the prover drew. The walk certifies
//! itself: it recomputes every query's DEEP value from its own challenges, so
//! a drifted copy cannot yield a plausible fixture. An independent verifier
//! porting the transcript anchors its known-answer tests here.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::poly::eval_cols_on_subgroup_ext;
use super::super::transcript::Transcript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::prove_ext::draw_ood_point_ext;
use super::spec::AirExt;
use super::types_ext::StarkProofExt;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

pub struct ReplayedExt {
    pub coeffs: Vec<Fp2>,
    pub z: Fp2,
    pub comp_z: Fp2,
    pub deep_coeffs: Vec<Fp2>,
    /// The consistency index of every query, in draw order.
    pub indices: Vec<usize>,
    /// Every query's DEEP value recomputed from the challenges above equals
    /// the proof's. False means this walk does not describe this proof.
    pub deep_consistent: bool,
}

pub fn replay_challenges_ext<A: AirExt>(
    air: &A,
    proof: &StarkProofExt,
    n_queries: usize,
) -> ReplayedExt {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, _) = domain_params_blown(air, 0);
    let n = 1usize << log_n;
    let window_size = air.window_size();
    let g = root_of_unity(log_t);
    let omega = root_of_unity(log_n);
    let shift = Fp::from_u64(SHIFT);

    let mut ts = Transcript::new(b"NONOS-STARK-EXT");
    ts.absorb_digest(&proof.trace_root);
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| ts.challenge_fp2()).collect();
    ts.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_ext(&mut ts, shift, n, t);
    for value in &proof.ood_frame {
        ts.absorb_fp(value.c0);
        ts.absorb_fp(value.c1);
    }
    let deep_coeffs: Vec<Fp2> =
        (0..width * window_size + 1).map(|_| ts.challenge_fp2()).collect();

    let periodic_z: Vec<Fp2> = eval_cols_on_subgroup_ext(g, t, &air.periodic_columns(), z);
    let comp_z = compose_ext(air, g, z, &proof.ood_frame, &periodic_z, &coeffs);
    ts.absorb_digest(&proof.fri.roots[0]);

    let mut indices = Vec::with_capacity(n_queries);
    let mut deep_consistent = true;
    for qd in proof.queries.iter().take(n_queries) {
        let p = ts.challenge_index(n);
        indices.push(p);
        let x = Fp2::from_base(shift * omega.pow(p as u64));
        let mut acc = Fp2::ZERO;
        for k in 0..window_size {
            let zk = z * Fp2::from_base(g.pow(k as u64));
            let inv_x_zk = (x - zk).inv();
            for c in 0..width {
                let claimed = proof.ood_frame[k * width + c];
                acc = acc
                    + deep_coeffs[k * width + c]
                        * ((Fp2::from_base(qd.trace[c]) - claimed) * inv_x_zk);
            }
        }
        let e = deep_coeffs[width * window_size];
        acc = acc + e * ((qd.comp - comp_z) * (x - z).inv());
        deep_consistent &= acc == qd.deep;
    }

    ReplayedExt { coeffs, z, comp_z, deep_coeffs, indices, deep_consistent }
}
