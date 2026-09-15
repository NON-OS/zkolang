// NONOS Operating System (AGPL-3.0-or-later)
//! The wire layout of the preprocessed-periodic proof: the frozen base proof
//! bytes (spec/stark-serialization.md), then the periodic sidecar — the
//! claims at z, and per consistency query the wide periodic row with its
//! path against the baked root.

use crate::crypto::stark::air::{serialize_proof_ext, StarkProofExtPre};
use alloc::vec::Vec;

/// The wire form a chain verifier decodes. Public because the settlement
/// artifact is written by a binary outside this module and must be written by
/// this encoder rather than a second one: two encoders for one format is the
/// way a proof that verifies in the prover fails to decode on the chain.
///
/// The base half is the library's own serializer rather than a copy. There was
/// a second copy inside the test-only generator, byte for byte the same; a
/// format with two writers only stays one format for as long as nobody edits
/// either, so the copy is gone and the test below holds this to the decoder.
pub fn serialize_pre(pre: &StarkProofExtPre) -> Vec<u8> {
    let mut b = serialize_proof_ext(&pre.proof);
    // sidecar: n_periodic, the claims at z, then one opening per query in
    // query order: n_periodic row values, then the path.
    b.extend_from_slice(&(pre.periodic_z.len() as u32).to_le_bytes());
    for v in &pre.periodic_z {
        b.extend_from_slice(&v.c0.value().to_le_bytes());
        b.extend_from_slice(&v.c1.value().to_le_bytes());
    }
    for op in &pre.openings {
        for v in &op.row {
            b.extend_from_slice(&v.value().to_le_bytes());
        }
        b.extend_from_slice(&(op.path.len() as u32).to_le_bytes());
        for d in &op.path {
            b.extend_from_slice(d);
        }
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::stark::air::deserialize_proof_ext;

    /*
     * The base half of the sidecar format is the plain proof encoding, and the
     * decoder on the other side reads it as one. Round tripping the prefix back
     * through the library's own parser is what pins that: if the base encoding
     * ever moves, this fails here rather than on a chain, and the sidecar's
     * offsets move with it rather than silently pointing into the wrong bytes.
     */
    #[test]
    fn the_base_half_is_the_plain_proof_encoding() {
        let (_air, pre, _root) = crate::preprocessed_tests::setup();
        let bytes = serialize_pre(&pre);
        let base = serialize_proof_ext(&pre.proof);
        assert_eq!(&bytes[..base.len()], &base[..], "the base half must be the proof encoding");
        let back = deserialize_proof_ext(&base).expect("the base half must parse on its own");
        assert_eq!(back.trace_root, pre.proof.trace_root);
        assert_eq!(back.comp_root, pre.proof.comp_root);
        assert_eq!(back.queries.len(), pre.proof.queries.len());
        assert!(bytes.len() > base.len(), "the sidecar must follow the base half");
    }
}
