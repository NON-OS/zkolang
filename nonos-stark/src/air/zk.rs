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

use super::super::field::{Fp, P};
use super::poseidon::{Poseidon, RATE};
use alloc::vec::Vec;

/// Draw a blinding seed from raw entropy: `RATE` field elements, uniform on the
/// field, consumed from `bytes` eight at a time in little-endian order. The caller
/// supplies the entropy (the OS CSPRNG in a capsule, the kernel entropy capability
/// in the kernel); this only turns bytes into a uniform field seed, so the sourcing
/// stays at the edge and the field draw stays testable.
///
/// Uniformity is the whole point of a blinding seed, so a raw eight-byte word that
/// lands at or above `P` is rejected and the next word tried, rather than reduced.
/// `Fp::from_u64` folds `[P, 2^64)` back onto `[0, 2^32 - 1)` with one subtraction,
/// which is a correct reduction but would make the low `2^32 - 1` residues twice as
/// likely; a biased seed weakens the hiding it is meant to provide. The rejection
/// rate is `(2^64 - P) / 2^64`, about `2^-32`, so the draw almost never skips.
///
/// `None` when `bytes` runs out before `RATE` words are accepted; the caller must
/// pass enough entropy, `RATE * 8` bytes plus a margin for the rare rejection.
pub fn seed_from_entropy(bytes: &[u8]) -> Option<[Fp; RATE]> {
    let mut out = [Fp::ZERO; RATE];
    let mut filled = 0usize;
    let mut i = 0usize;
    while filled < RATE {
        if i + 8 > bytes.len() {
            return None;
        }
        let mut word = [0u8; 8];
        word.copy_from_slice(&bytes[i..i + 8]);
        i += 8;
        let v = u64::from_le_bytes(word);
        if v < P {
            // v is already canonical, so from_u64 returns it unchanged.
            out[filled] = Fp::from_u64(v);
            filled += 1;
        }
    }
    Some(out)
}

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

    /// The seed draw is a deterministic function of the entropy bytes and lands
    /// canonical: same bytes, same seed, every element below `P`.
    #[test]
    fn a_seed_draw_is_deterministic_and_canonical() {
        let mut bytes = Vec::new();
        for k in 0..RATE * 8 {
            bytes.push((7 * k + 1) as u8);
        }
        let a = seed_from_entropy(&bytes).expect("enough entropy");
        let b = seed_from_entropy(&bytes).expect("enough entropy");
        assert_eq!(a, b, "the same bytes gave two different seeds");
        for e in a.iter() {
            assert!(e.value() < P, "a drawn element is not canonical");
        }
    }

    /// An eight-byte word at or above `P` is skipped, not folded: the seed drawn
    /// from a rejected word followed by good words equals the seed from the good
    /// words alone, so the rejection keeps the draw uniform instead of biasing the
    /// low residues.
    #[test]
    fn an_out_of_range_word_is_rejected_not_folded() {
        let mut good = Vec::new();
        for k in 0..RATE * 8 {
            good.push((3 * k + 5) as u8);
        }
        // u64::MAX is above P, so the leading word must be skipped.
        let mut with_reject = Vec::new();
        with_reject.extend_from_slice(&u64::MAX.to_le_bytes());
        with_reject.extend_from_slice(&good);

        let from_good = seed_from_entropy(&good).expect("enough entropy");
        let from_reject = seed_from_entropy(&with_reject).expect("enough entropy");
        assert_eq!(
            from_good, from_reject,
            "the out-of-range word was folded in rather than skipped"
        );
    }

    /// Too few bytes to fill `RATE` accepted words is a refusal, not a short or
    /// zero-padded seed.
    #[test]
    fn insufficient_entropy_refuses() {
        let bytes = alloc::vec![0u8; RATE * 8 - 1];
        assert!(seed_from_entropy(&bytes).is_none(), "a short entropy buffer must refuse");
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

    /// The same, on the Poseidon prover the deployed transfer runs on: a blinded
    /// proof verifies under the plain Poseidon verifier and its out-of-domain
    /// frame moves, so the proof over the real commitment path hides too.
    #[test]
    fn a_blinded_poseidon_proof_verifies_and_moves_the_frame() {
        use crate::air::{
            stark_prove_poseidon_ext, stark_prove_poseidon_ext_zk, stark_verify_poseidon_ext_pub,
            Air, Squaring,
        };

        let h = hasher();
        let air = Squaring { log_t: 5, seed: Fp::from_u64(3) };
        let t = 1usize << 5;
        let mut trace = Vec::with_capacity(t);
        let mut x = Fp::from_u64(3);
        for _ in 0..t {
            trace.push(x);
            x = x * x;
        }

        let nq = 8;
        let plain = stark_prove_poseidon_ext(&air, &trace, nq, 0, 0, &h);
        assert!(
            stark_verify_poseidon_ext_pub(&air, &plain, nq, 0, 0, &h, &[]),
            "the plain poseidon proof did not verify"
        );

        let bseed = [Fp::from_u64(5), Fp::from_u64(6), Fp::from_u64(7), Fp::from_u64(8)];
        let blind: Vec<Vec<Fp>> =
            (0..air.trace_width()).map(|c| blinding_poly(&h, &bseed, c, nq)).collect();
        let zk = stark_prove_poseidon_ext_zk(&air, &trace, nq, 0, 0, &h, &[], &blind);

        assert!(
            stark_verify_poseidon_ext_pub(&air, &zk, nq, 0, 0, &h, &[]),
            "the blinded poseidon proof did not verify"
        );
        assert_ne!(plain.ood_frame, zk.ood_frame, "blinding did not move the frame");
    }
}
