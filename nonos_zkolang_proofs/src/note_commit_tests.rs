/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The Poseidon note commitment, cross-checked against its reference. The circuit
//! in circuits/shield/note_commit.zkl reimplements the width-8 Poseidon-Goldilocks
//! commit_note from nonos-stark: four full rounds, the Cauchy MDS, the BLAKE3 round
//! constants, and the same eleven-limb-plus-domain layout. These tests run the
//! circuit on a witness and require its revealed digest to equal, field element for
//! field element, what nonos-stark computes for the same input. A commitment that
//! disagreed with the vetted hash would be a broken commitment, so this equality is
//! the thing that makes the in-language version trustworthy.

use std::fs;
use std::path::PathBuf;

use nonos_stark::air::{Poseidon, NOTE_LIMBS, RATE};
use nonos_stark::field::Fp;
use nonos_zkolang::{compile_source, evaluate};

fn circuit() -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("circuits/shield/note_commit.zkl");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

// The reference: nonos-stark's own note commitment over eleven limbs.
fn reference(limbs: &[u64; NOTE_LIMBS]) -> Vec<u64> {
    let hasher = Poseidon::new(2, [Fp::ZERO; RATE]);
    let fps: [Fp; NOTE_LIMBS] = core::array::from_fn(|i| Fp::from_u64(limbs[i]));
    hasher.commit_note(&fps).iter().map(|f| f.value()).collect()
}

// The circuit: compile it once and run it on the witness, no proving needed to read
// the revealed outputs. The circuit has no ordered comparisons, so there is no advice
// to fill and a plain evaluation is exact.
fn in_language(limbs: &[u64; NOTE_LIMBS]) -> Vec<u64> {
    let ops = compile_source(&circuit()).expect("compile note_commit.zkl");
    evaluate(&ops, &[], limbs).expect("run note_commit.zkl")
}

fn check(limbs: [u64; NOTE_LIMBS]) {
    let want = reference(&limbs);
    let got = in_language(&limbs);
    assert_eq!(want.len(), RATE);
    assert_eq!(
        got, want,
        "note commitment for {limbs:?} must match nonos-stark commit_note"
    );
}

#[test]
fn matches_reference_on_counting_limbs() {
    check([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
}

#[test]
fn matches_reference_on_zero() {
    check([0; NOTE_LIMBS]);
}

#[test]
fn matches_reference_on_large_field_values() {
    // Values near the modulus exercise the field reductions in both implementations.
    check([
        18446744069414584320,
        9223372034707292160,
        1,
        18446744069414584320,
        123456789,
        0,
        18446744069414584319,
        7,
        18446744069414584320,
        11,
        18446744069414584318,
    ]);
}

#[test]
fn commitment_is_deterministic() {
    let a = in_language(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]);
    let b = in_language(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]);
    assert_eq!(a, b);
}

#[test]
fn distinct_limbs_give_distinct_commitments() {
    let a = in_language(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    let b = in_language(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12]);
    assert_ne!(a, b, "changing a limb must change the commitment");
}
