// NONOS Operating System (AGPL-3.0-or-later)
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

//! The preprocessed transcript's prefix, replayed to the composition at z.
//!
//! `stark_verify_ext_preprocessed` draws the composition coefficients and the
//! out-of-domain point before it looks at a query, and the point it draws
//! depends on the evaluation domain, which depends on the rate the proof was
//! made at. `replay_challenges_ext` replays the unblown transcript, so for a
//! proof at any other rate its z is a point the proof never opened and its
//! `comp_z` is the composition there. This replays the same prefix at the
//! proof's own rate and stops once `comp_z` is known, because that is the one
//! value a chain verifier is still handed rather than derives, and the value
//! an emitter has to print beside the proof it belongs to.
//!
//! Nothing here is a second opinion. The transcript label, the draw order and
//! the composition are the verifier's own calls in the verifier's own order;
//! a proof this disagrees with is a proof the verifier rejects.

use super::super::field::{Fp, Fp2};
use super::super::fri::root_of_unity;
use super::super::transcript::Transcript;
use super::composition::{compose_ext, domain_params_blown, num_coeffs};
use super::prove_ext::{draw_ood_point_ext, COSET_SHIFT};
use super::spec::AirExt;
use super::types_ext_pre::StarkProofExtPre;
use alloc::vec::Vec;

/// What the transcript had drawn by the time the composition at z was fixed.
pub struct ReplayedPre {
    /// The composition coefficients, in draw order: transitions first, then
    /// boundaries, the order the emitted lists are paired against.
    pub coeffs: Vec<Fp2>,
    pub z: Fp2,
    pub comp_z: Fp2,
}

/// Replay the prefix of the preprocessed verifier at `extra_blowup_bits`, the
/// rate the proof was made at, under the statement's `publics`, and return
/// the composition it determines.
pub fn replay_comp_z_pre<A: AirExt>(
    air: &A,
    pre: &StarkProofExtPre,
    extra_blowup_bits: u32,
    publics: &[Fp],
) -> ReplayedPre {
    let proof = &pre.proof;
    let log_t = air.log_trace_len();
    let t = 1usize << log_t;
    let (log_n, _) = domain_params_blown(air, extra_blowup_bits);
    let n = 1usize << log_n;
    let g = root_of_unity(log_t);
    let shift = Fp::from_u64(COSET_SHIFT);

    let mut ts = Transcript::new(b"NONOS-STARK-EXT");
    for value in publics {
        ts.absorb_fp(*value);
    }
    ts.absorb_digest(&proof.trace_root);
    let coeffs: Vec<Fp2> = (0..num_coeffs(air)).map(|_| ts.challenge_fp2()).collect();
    ts.absorb_digest(&proof.comp_root);
    let z = draw_ood_point_ext(&mut ts, shift, n, t);
    let comp_z = compose_ext(air, g, z, &proof.ood_frame, &pre.periodic_z, &coeffs);

    ReplayedPre { coeffs, z, comp_z }
}
