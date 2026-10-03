/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A format 7 proof verified by `nox_verify`, the verifier the STARKs gates link. It takes
 * a statement of pinned statics, as a gate compiles them in; a statement made here is
 * pinned once per process, so verifying it again allocates nothing.
 */

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use nox_verify::{Point, Refusal};

use super::params::{shape, EXTRA_BLOWUP_BITS, GRIND_BITS, MAX_PROOF_BYTES, QUERIES};
use super::statement::Statement;

type Pinned = BTreeMap<(Vec<u8>, [[u8; 32]; 3], usize), &'static nox_verify::Statement>;

static PINNED: OnceLock<Mutex<Pinned>> = OnceLock::new();

/** The statement `nox_verify` reads, pinned once per process for each set of pins. */
fn pinned(image: &[u8], pins: [[u8; 32]; 3], words: usize) -> &'static nox_verify::Statement {
    let key = (image.to_vec(), pins, words);
    let mut all = PINNED
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let point = Point {
        params: pins[2],
        queries: QUERIES,
        grind_bits: GRIND_BITS,
    };
    all.entry(key).or_insert_with(|| {
        Box::leak(Box::new(nox_verify::Statement {
            program: Box::leak(image.to_vec().into_boxed_slice()),
            program_hash: pins[0],
            periodic_root: pins[1],
            shape: shape(),
            extra_blowup_bits: EXTRA_BLOWUP_BITS,
            points: Box::leak(Box::new([point])),
            words,
            max_proof_bytes: MAX_PROOF_BYTES,
        }))
    })
}

/** `proof` verified by `nox_verify` against `st` and the public `words`; why, if not. */
pub fn verify(st: &Statement, proof: &[u8], words: &[u64]) -> Result<(), (Refusal, &'static str)> {
    let pins = [st.image_hash, st.periodic_root, st.params];
    verify_pinned(&st.image, pins, st.words, proof, words)
}

/**
 * `proof` verified as `verify` does, against pins a verifier holds apart from any program:
 * the image, its hash, the periodic root and the parameter identity, and the word count.
 */
pub fn verify_pinned(
    image: &[u8],
    pins: [[u8; 32]; 3],
    n_words: usize,
    proof: &[u8],
    words: &[u64],
) -> Result<(), (Refusal, &'static str)> {
    nox_verify::verify_why(pinned(image, pins, n_words), proof, words)
}
