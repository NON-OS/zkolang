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

//! Prover-side zero-knowledge randomness. The blinding polynomial `r` that
//! `poly::blind_coeffs` adds as `r * Z_H` has to be the prover's own secret. It is
//! never absorbed into the transcript: a verifier that could recompute `r` could
//! subtract `r * Z_H` from the openings and recover the witness, so the hiding
//! rests entirely on `r` staying private. This expands a secret seed into a
//! blinding polynomial per column with Poseidon in counter mode. Fresh entropy in
//! the seed each proof makes the blinding, and therefore the query openings,
//! fresh; a deterministic seed reproduces a proof for a test without weakening the
//! construction, since the seed is the secret either way.

use super::super::field::Fp;
use super::poseidon::{Poseidon, RATE};
use alloc::vec::Vec;

/// The blinding polynomial for one column: `deg + 1` coefficients, low degree
/// first, expanded from `seed` by Poseidon in counter mode. A distinct
/// `(column, counter)` tag per block keeps the columns' blindings independent, so
/// one opened column says nothing about another's blinding.
pub fn blinding_poly(h: &Poseidon, seed: &[Fp; RATE], column: usize, deg: usize) -> Vec<Fp> {
    let mut out = Vec::with_capacity(deg + 1);
    let mut ctr = 0u64;
    while out.len() <= deg {
        let mut tag = [Fp::ZERO; RATE];
        tag[0] = Fp::from_u64(column as u64);
        tag[1] = Fp::from_u64(ctr);
        for &x in h.compress(seed, &tag).iter() {
            if out.len() <= deg {
                out.push(x);
            }
        }
        ctr += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hasher() -> Poseidon {
        Poseidon::new(5, [Fp::ZERO; RATE])
    }

    /// The blinding has the requested degree and is a deterministic function of
    /// the seed: the same seed reproduces the same proof, which a test needs.
    #[test]
    fn deterministic_and_the_right_length() {
        let h = hasher();
        let seed = [Fp::from_u64(11), Fp::from_u64(22), Fp::from_u64(33), Fp::from_u64(44)];
        let a = blinding_poly(&h, &seed, 0, 8);
        let b = blinding_poly(&h, &seed, 0, 8);
        assert_eq!(a.len(), 9);
        assert_eq!(a, b, "same seed did not reproduce the same blinding");
    }

    /// Different columns, and different seeds, give different blindings: the
    /// per-column tag separates them, and the seed is what varies the whole thing
    /// from one proof to the next.
    #[test]
    fn independent_across_columns_and_seeds() {
        let h = hasher();
        let seed = [Fp::from_u64(11), Fp::from_u64(22), Fp::from_u64(33), Fp::from_u64(44)];
        let other = [Fp::from_u64(99), Fp::from_u64(22), Fp::from_u64(33), Fp::from_u64(44)];
        let col0 = blinding_poly(&h, &seed, 0, 8);
        let col1 = blinding_poly(&h, &seed, 1, 8);
        let seed2 = blinding_poly(&h, &other, 0, 8);
        assert_ne!(col0, col1, "two columns share a blinding");
        assert_ne!(col0, seed2, "two seeds share a blinding");
    }

    /// End to end: a blinded proof verifies under the plain verifier, which is the
    /// whole point, the blinding is invisible to it, and the proof differs from the
    /// unblinded one, so the openings genuinely moved. Squaring is degree two, so
    /// its composition bound leaves room for the higher-degree blinded columns.
    #[test]
    fn a_blinded_proof_verifies_and_moves_the_openings() {
        use crate::air::{
            serialize_proof_ext, stark_prove_ext, stark_prove_ext_zk, stark_verify_ext, Air,
            Squaring,
        };

        let log_t = 5u32; // t = 32; degree 2 gives a bound of 64, room for a degree-8 blind
        let seed_v = Fp::from_u64(3);
        let air = Squaring { log_t, seed: seed_v };
        let t = 1usize << log_t;
        let mut trace = Vec::with_capacity(t);
        let mut x = seed_v;
        for _ in 0..t {
            trace.push(x);
            x = x * x;
        }

        let nq = 8;
        let plain = stark_prove_ext(&air, &trace, nq, 0);
        assert!(stark_verify_ext(&air, &plain, nq, 0), "the plain proof did not verify");

        let h = hasher();
        let seed = [Fp::from_u64(1), Fp::from_u64(2), Fp::from_u64(3), Fp::from_u64(4)];
        let blind: Vec<Vec<Fp>> =
            (0..air.trace_width()).map(|c| blinding_poly(&h, &seed, c, nq)).collect();
        let zk = stark_prove_ext_zk(&air, &trace, nq, 0, 0, &blind);

        assert!(stark_verify_ext(&air, &zk, nq, 0), "the blinded proof did not verify");
        assert_ne!(
            serialize_proof_ext(&plain),
            serialize_proof_ext(&zk),
            "blinding did not change the proof"
        );
    }
}
