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

//! Measuring a capsule or kernel image to a Poseidon leaf. The bytes are absorbed
//! into a Poseidon sponge seven at a time (each block a canonical field element) and
//! the rate lanes are squeezed to a digest. The length is bound first so images of
//! different size cannot collide. That digest is the enrolled leaf: a policy root
//! commits to exactly the measured images, and an attestation proves membership of a
//! real measurement rather than an arbitrary secret.

use super::super::field::Fp;
use super::poseidon::{Poseidon, RATE, WIDTH};

/// The Poseidon measurement of `image`: bind the length, absorb the bytes in
/// seven-byte little-endian blocks with one permutation per rate group, then squeeze
/// the rate lanes.
pub fn measure_capsule(hasher: &Poseidon, image: &[u8]) -> [Fp; RATE] {
    let mut state = [Fp::ZERO; WIDTH];
    state[0] = state[0] + Fp::from_u64(image.len() as u64);
    state = hasher.permute(state);

    let mut lane = 0usize;
    let mut i = 0usize;
    while i < image.len() {
        let take = core::cmp::min(7, image.len() - i);
        let mut buf = [0u8; 8];
        buf[..take].copy_from_slice(&image[i..i + take]);
        state[lane] = state[lane] + Fp::from_u64(u64::from_le_bytes(buf));
        lane += 1;
        if lane == RATE {
            state = hasher.permute(state);
            lane = 0;
        }
        i += 7;
    }
    if lane != 0 {
        state = hasher.permute(state);
    }

    let mut digest = [Fp::ZERO; RATE];
    digest.copy_from_slice(&state[..RATE]);
    digest
}

/// The domain separator for the hybrid measurement, absorbed before the digest
/// so a hybrid leaf can never equal a direct leaf of the same bytes.
const HYBRID_DOMAIN: u64 = 0x4E4F_4E4F_5342_3348; // "NONOSB3H"

/// The hybrid measurement: BLAKE3 the image, then absorb only the 32-byte
/// digest into the sponge, one permutation.
///
/// The direct measurement above is a sponge over every byte of the image. That
/// is the cost of an enrolment, once, and it would be the cost of every spawn
/// once the verifier measures the image itself, which it must. Both gates
/// already hold a BLAKE3 measurement of what they are about to run, so the leaf
/// costs them one permutation, and the scheme rests on BLAKE3 collision
/// resistance, which their context already assumed.
///
/// The two schemes are domain separated, so a root built from one never
/// verifies a trailer built from the other. Changing how a set is measured has
/// to invalidate the set, not quietly reinterpret it.
pub fn measure_capsule_hybrid(hasher: &Poseidon, image: &[u8]) -> [Fp; RATE] {
    let digest = blake3::hash(image);
    let bytes = digest.as_bytes();

    let mut state = [Fp::ZERO; WIDTH];
    state[0] = state[0] + Fp::from_u64(HYBRID_DOMAIN);
    /*
     * Four eight-byte words, each reduced into the field. The reduction folds
     * the top 2^32 - 1 values of a word onto the bottom ones, so one lane can
     * collide between two digests; all four together still carry close to 256
     * bits, and a collision across all four is the collision BLAKE3 rules out.
     */
    for (lane, word) in bytes.chunks(8).enumerate() {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(word);
        state[lane + 1] = state[lane + 1] + Fp::from_u64(u64::from_le_bytes(buf));
    }
    state = hasher.permute(state);

    let mut out = [Fp::ZERO; RATE];
    out.copy_from_slice(&state[..RATE]);
    out
}
