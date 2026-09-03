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

//! The poseidon transcript replay, written once. Every consumer that rederives
//! the prover's challenges walked this sequence as its own copy: the deep-term
//! builders, the opening builders, the verifiers, plain and preprocessed. Five
//! copies of one walk is five places a drift can hide, and the sidecar found
//! one. The walk lives here; a consumer says whether claims ride the proof and
//! where it wants to stop.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::poseidon_transcript::PoseidonTranscript;
use super::composition::{domain_params_blown, num_coeffs};
use super::draw_ood_poseidon::draw_ood_point_poseidon;
use super::poseidon::Poseidon;
use super::spec::AirExt;
use super::types_poseidon_ext::StarkProofExtP;
use alloc::vec::Vec;

const SHIFT: u64 = 7;

/// Everything the transcript yields up to and including the DEEP coefficients,
/// with the transcript positioned right after the first FRI root, ready for
/// the query index draws.
pub struct Replayed {
    pub coeffs: Vec<Fp2>,
    pub z: Fp2,
    pub deep_coeffs: Vec<Fp2>,
    pub ts: PoseidonTranscript,
}

/// Replay the prover's walk over `proof`, bound to `publics`. `claims` is the
/// periodic sidecar when the proof carries one: absorbed after the frame, and
/// the coefficient draw widens with it. `None` is the plain path, bit for bit
/// the sequence it always was.
pub fn replay<A: AirExt>(
    air: &A,
    proof: &StarkProofExtP,
    claims: Option<&[Fp2]>,
    extra_blowup_bits: u32,
    hasher: &Poseidon,
    publics: &[Fp],
) -> Replayed {
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let width = air.trace_width();
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let window_size = air.window_size();
    let n_claims = claims.map(|c| c.len()).unwrap_or(0);

    let mut ts = PoseidonTranscript::new(hasher.clone());
    for &p in publics {
        ts.absorb(p);
    }
    ts.absorb_digest(&proof.trace_root);
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| ts.challenge_fp2()).collect();
    ts.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_poseidon(&mut ts, Fp::from_u64(SHIFT), n, t);
    for value in &proof.ood_frame {
        ts.absorb(value.c0);
        ts.absorb(value.c1);
    }
    if let Some(cl) = claims {
        for value in cl {
            ts.absorb(value.c0);
            ts.absorb(value.c1);
        }
    }
    let deep_coeffs: Vec<Fp2> = (0..width * window_size + 1 + n_claims)
        .map(|_| ts.challenge_fp2())
        .collect();
    ts.absorb_digest(&proof.fri.roots[0]);

    Replayed {
        coeffs,
        z,
        deep_coeffs,
        ts,
    }
}

/// The k-th consistency index after a replay: the first `k` draws consumed and
/// discarded, matching the verifier's loop.
pub fn query_index(r: &mut Replayed, n: usize, query: usize) -> usize {
    for _ in 0..query {
        r.ts.challenge_index(n);
    }
    r.ts.challenge_index(n)
}

/// The evaluation domain size the replay's index draws range over.
pub fn domain_size<A: AirExt>(air: &A, extra_blowup_bits: u32) -> usize {
    1usize << domain_params_blown(air, extra_blowup_bits).0
}

/// The trace-domain generator, for consumers composing at z.
pub fn trace_generator<A: AirExt>(air: &A) -> Fp {
    root_of_unity(air.log_trace_len())
}
