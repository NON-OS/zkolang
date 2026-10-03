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

use nox_verify::{Point, Refusal, Shape};

use super::params::{dims, EXTRA_BLOWUP_BITS, GRIND_BITS, LANES, MAX_PROOF_BYTES, QUERIES};
use super::statement::Statement;

type Pins = BTreeMap<(Vec<u8>, [u8; 32], [u8; 32], usize), &'static nox_verify::Statement>;

static PINNED: OnceLock<Mutex<Pins>> = OnceLock::new();

/** The numbers of a zKølang statement the image does not carry. */
pub fn shape() -> Shape {
    let (trace_width, window, constraint_degree) = dims();
    Shape {
        trace_width,
        region_width: trace_width - 1,
        window,
        constraint_degree,
        mask_pair: None,
        challenge_lanes: LANES,
    }
}

fn pinned(st: &Statement) -> &'static nox_verify::Statement {
    let key = (st.image.clone(), st.periodic_root, st.params, st.words);
    let mut pins = PINNED
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let point = Point {
        params: st.params,
        queries: QUERIES,
        grind_bits: GRIND_BITS,
    };
    pins.entry(key).or_insert_with(|| {
        Box::leak(Box::new(nox_verify::Statement {
            program: Box::leak(st.image.clone().into_boxed_slice()),
            program_hash: st.image_hash,
            periodic_root: st.periodic_root,
            shape: shape(),
            extra_blowup_bits: EXTRA_BLOWUP_BITS,
            points: Box::leak(Box::new([point])),
            words: st.words,
            max_proof_bytes: MAX_PROOF_BYTES,
        }))
    })
}

/** `proof` verified by `nox_verify` against `st` and the public `words`; why, if not. */
pub fn verify(st: &Statement, proof: &[u8], words: &[u64]) -> Result<(), (Refusal, &'static str)> {
    nox_verify::verify_why(pinned(st), proof, words)
}
