/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A blinding seed from bytes the caller draws from a source of randomness. */

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

/** How many random bytes a seed takes: eight for each of its elements. */
pub const SEED_BYTES: usize = 8 * RATE;

/**
 * The seed whose elements are read from `bytes`, eight little-endian bytes each, reduced
 * into the field. The bytes must be fresh and secret for the proof to hide the witness.
 */
pub fn seed_of(bytes: &[u8; SEED_BYTES]) -> [Fp; RATE] {
    let mut seed = [Fp::ZERO; RATE];
    let (chunks, _) = bytes.as_chunks::<8>();
    for (s, chunk) in seed.iter_mut().zip(chunks) {
        *s = Fp::from_u64(u64::from_le_bytes(*chunk));
    }
    seed
}
