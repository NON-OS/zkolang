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

//! The one verify path both the capsule spawn gate and the kernel self-attestation
//! share. It parses an attestation trailer against a trusted root, measures the
//! image the caller is about to run, and checks the money-grade proof that this
//! measurement sits under that root, bound to the caller's context. The root is
//! always the caller's, never the trailer's, and so is the leaf: the trailer
//! carries the path and the proof, the verifier supplies both ends of the path.
//!
//! The leaf has to come from the verifier. A proof over a private leaf says only
//! that some enrolled leaf sits under the root, and every enrolled leaf is a
//! public function of a shipped image, with its path shipped in the clear beside
//! it. Anyone holding one release image could mint such a proof for any image
//! under any context. Pinning the measurement of the image actually being
//! admitted is what makes the proof about that image.

use super::measure::measure_capsule_hybrid;
use super::poseidon::{Poseidon, RATE};
use super::{deserialize_proof_ext, stark_verify_ext_blown_bound, MultiMembership, Opening};
use crate::field::Fp;
use alloc::vec::Vec;

/// The trailer magic, shared with the builder.
pub const TRAILER_MAGIC: &[u8; 8] = b"NZKSTRK1";

/// Read four little-endian words into a rate-width Poseidon digest.
fn to_rate(bytes: &[u8]) -> [Fp; RATE] {
    let mut out = [Fp::ZERO; RATE];
    for (i, lane) in out.iter_mut().enumerate() {
        let mut w = [0u8; 8];
        w.copy_from_slice(&bytes[i * 8..i * 8 + 8]);
        *lane = Fp::from_u64(u64::from_le_bytes(w));
    }
    out
}

/// Verify a `depth`-level membership trailer for `image` against `root`, bound
/// to `context`, at the given soundness. The image is measured here, the hybrid
/// way, and that measurement is the leaf the proof must open. False on any
/// malformed byte or any failed check, so an attestation on a hostile trailer
/// fails cleanly rather than panicking.
#[allow(clippy::too_many_arguments)]
pub fn verify_membership_trailer(
    hasher: &Poseidon,
    log_rounds: u32,
    root: [u8; 32],
    image: &[u8],
    depth: usize,
    trailer: &[u8],
    context: &[u8],
    n_queries: usize,
    grind_bits: u32,
    extra_blowup_bits: u32,
) -> bool {
    let dir_bytes = depth.div_ceil(8);
    let sib_end = 9 + depth * 32;
    if trailer.len() < sib_end + dir_bytes
        || &trailer[0..8] != TRAILER_MAGIC
        || trailer[8] as usize != depth
    {
        return false;
    }

    let mut siblings = Vec::with_capacity(depth);
    for i in 0..depth {
        siblings.push(to_rate(&trailer[9 + i * 32..9 + i * 32 + 32]));
    }
    let dirs = &trailer[sib_end..sib_end + dir_bytes];
    let directions: Vec<bool> = (0..depth)
        .map(|i| (dirs[i / 8] >> (i % 8)) & 1 == 1)
        .collect();
    let Some(proof) = deserialize_proof_ext(&trailer[sib_end + dir_bytes..]) else {
        return false;
    };

    /*
     * One opening on the batch gadget rather than the single-opening one: the
     * batch gadget pins the whole initial state of its first opening, leaf and
     * sibling both, where the single-opening gadget pins the sibling half only.
     */
    let opening = Opening {
        leaf: measure_capsule_hybrid(hasher, image),
        root: to_rate(&root),
        siblings,
        directions,
    };
    let air = MultiMembership::new(hasher.clone(), log_rounds, alloc::vec![opening]);
    stark_verify_ext_blown_bound(
        &air,
        &proof,
        n_queries,
        grind_bits,
        extra_blowup_bits,
        context,
    )
}
