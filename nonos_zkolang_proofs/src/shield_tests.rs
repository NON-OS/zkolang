/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The shielded note spend, proven end to end. A valid note spends and its nullifier is
//! deterministic; a spend against the wrong root has no proof, and a value that is not a
//! byte has no proof. This is the private-value utility, whole, with its soundness gates.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_witness};

fn base() -> PathBuf {
    let mut b = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    b.pop();
    b
}

fn resolve(path: &str) -> Option<String> {
    fs::read_to_string(base().join("stdlib").join(path)).ok()
}

fn prog(rel: &str) -> String {
    let src = fs::read_to_string(base().join(rel)).expect("read");
    expand_includes(&src, &mut resolve).expect("expand")
}

// The witness of a note: value 42, key 7, siblings 11 13 17, path 0 1 0.
const NOTE: [u64; 8] = [42, 7, 11, 13, 17, 0, 1, 0];
// The eight bits of the value 42.
const BITS: [u64; 8] = [0, 1, 0, 1, 0, 1, 0, 0];

fn note_root() -> u64 {
    prove_source_with_witness(&prog("circuits/shield/note_root.zkl"), &[], &NOTE)
        .expect("root")
        .outputs[0]
}

fn spend(public: &[u64], witness: &[u64]) -> bool {
    prove_source_with_witness(&prog("circuits/shield/spend_note.zkl"), public, witness)
        .map(|r| r.verified)
        .unwrap_or(false)
}

#[test]
fn a_valid_note_spends_and_its_nullifier_is_deterministic() {
    let root = note_root();
    let witness: Vec<u64> = NOTE.iter().chain(BITS.iter()).copied().collect();
    let a = prove_source_with_witness(
        &prog("circuits/shield/spend_note.zkl"),
        &[root, 99],
        &witness,
    )
    .expect("spend");
    assert!(a.verified);
    let b = prove_source_with_witness(
        &prog("circuits/shield/spend_note.zkl"),
        &[root, 99],
        &witness,
    )
    .expect("spend");
    assert_eq!(a.outputs, b.outputs, "the nullifier is not deterministic");
    // A different position gives a different nullifier for the same note.
    let c = prove_source_with_witness(
        &prog("circuits/shield/spend_note.zkl"),
        &[root, 100],
        &witness,
    )
    .expect("spend");
    assert_ne!(a.outputs, c.outputs);
}

#[test]
fn a_spend_against_the_wrong_root_has_no_proof() {
    let root = note_root();
    let witness: Vec<u64> = NOTE.iter().chain(BITS.iter()).copied().collect();
    assert!(
        !spend(&[root + 1, 99], &witness),
        "membership under a wrong root proved"
    );
}

#[test]
fn a_value_that_is_not_a_byte_has_no_proof() {
    // The value is 42 but the bit witness composes to 255, so the range constraint fails.
    let root = note_root();
    let bad: Vec<u64> = NOTE
        .iter()
        .copied()
        .chain([1, 1, 1, 1, 1, 1, 1, 1])
        .collect();
    assert!(
        !spend(&[root, 99], &bad),
        "a value inconsistent with its bits proved"
    );
}

// The full transfer note: input value 50, key 7, siblings 11 13 17, path 0 1 0, then the
// output recipient key 9 and blinding 3. The tree root is over the input note.
fn transfer_root() -> u64 {
    prove_source_with_witness(
        &prog("circuits/shield/note_root.zkl"),
        &[],
        &[50, 7, 11, 13, 17, 0, 1, 0],
    )
    .expect("root")
    .outputs[0]
}

fn transfer(public: &[u64], witness: &[u64]) -> bool {
    prove_source_with_witness(&prog("circuits/shield/transfer_note.zkl"), public, witness)
        .map(|r| r.verified)
        .unwrap_or(false)
}

#[test]
fn a_confidential_transfer_spends_and_creates_conserving_value() {
    let root = transfer_root();
    // witness: in_value,in_key, siblings, path, out_key, out_blind, then bits of 42.
    let w: Vec<u64> = [50, 7, 11, 13, 17, 0, 1, 0, 9, 3]
        .into_iter()
        .chain([0, 1, 0, 1, 0, 1, 0, 0])
        .collect();
    // Spend 50 with fee 8: output value 42, a byte. Reveals the nullifier and output note.
    let r = prove_source_with_witness(
        &prog("circuits/shield/transfer_note.zkl"),
        &[root, 99, 8],
        &w,
    )
    .expect("transfer");
    assert!(r.verified);
    assert_eq!(
        r.outputs.len(),
        2,
        "a transfer reveals a nullifier and an output note"
    );
}

#[test]
fn a_transfer_whose_fee_exceeds_the_input_has_no_proof() {
    // Fee 60 on an input of 50 would mint value; the output value wraps out of range.
    let root = transfer_root();
    let w: Vec<u64> = [50, 7, 11, 13, 17, 0, 1, 0, 9, 3]
        .into_iter()
        .chain([0, 1, 0, 1, 0, 1, 0, 0])
        .collect();
    assert!(
        !transfer(&[root, 99, 60], &w),
        "a value-minting transfer proved"
    );
}
