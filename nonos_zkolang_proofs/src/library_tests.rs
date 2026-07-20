/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The example programs and the standard library they include, each proven. Every
//! program in examples/ is read, its includes are resolved from stdlib/, and it is run
//! on representative inputs. The arithmetic programs check an exact output; the shield
//! programs check that a valid witness proves and, where it applies, that an invalid
//! one does not. Nothing here is Rust logic: it is zKolang source the compiler lowers
//! and the STARK proves.

use std::fs;
use std::path::PathBuf;

use nonos_zkolang::{expand_includes, prove_source_with_witness};

fn root() -> PathBuf {
    let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base.pop();
    base
}

fn resolve(path: &str) -> Option<String> {
    fs::read_to_string(root().join("stdlib").join(path)).ok()
}

// Expand a program's includes and prove it on public inputs and a private witness.
// Returns whether the run produced a verifying proof.
fn proves(file: &str, inputs: &[u64], witness: &[u64]) -> bool {
    let src = fs::read_to_string(root().join("examples").join(file)).expect("read");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    prove_source_with_witness(&expanded, inputs, witness)
        .map(|r| r.verified)
        .unwrap_or(false)
}

// The single output of a proven program.
fn out(file: &str, inputs: &[u64], witness: &[u64]) -> u64 {
    let src = fs::read_to_string(root().join("examples").join(file)).expect("read");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let report = prove_source_with_witness(&expanded, inputs, witness).expect("run");
    assert!(report.verified);
    report.outputs[0]
}

fn outs(file: &str, inputs: &[u64], witness: &[u64]) -> Vec<u64> {
    let src = fs::read_to_string(root().join("examples").join(file)).expect("read");
    let expanded = expand_includes(&src, &mut resolve).expect("expand");
    let report = prove_source_with_witness(&expanded, inputs, witness).expect("run");
    assert!(report.verified);
    report.outputs
}

#[test]
fn arithmetic_and_sequence_programs() {
    assert_eq!(out("quartic.zkl", &[3], &[]), 81);
    assert_eq!(out("lucas.zkl", &[], &[]), 123);
    assert_eq!(out("tribonacci.zkl", &[], &[]), 81);
    assert_eq!(out("triangular.zkl", &[], &[]), 55);
    assert_eq!(out("sum_of_squares.zkl", &[], &[]), 55);
    assert_eq!(out("norm_sq.zkl", &[2, 3, 4], &[]), 29);
    assert_eq!(outs("matmul2.zkl", &[], &[]), vec![19, 22, 43, 50]);
}

#[test]
fn boolean_circuit_programs() {
    assert_eq!(outs("full_adder.zkl", &[1, 1, 0], &[]), vec![0, 1]);
    assert_eq!(outs("full_adder.zkl", &[1, 1, 1], &[]), vec![1, 1]);
    assert_eq!(out("popcount4.zkl", &[1, 0, 1, 1], &[]), 3);
    assert_eq!(out("mux4.zkl", &[10, 20, 30, 40, 0, 1], &[]), 30);
}

#[test]
fn shielded_value_programs() {
    // Value conservation proves when balanced and has no proof otherwise.
    assert!(proves("transfer.zkl", &[10, 5, 8, 4, 3], &[]));
    assert!(!proves("transfer.zkl", &[10, 5, 8, 4, 4], &[]));

    // A byte range proof: the correct bit witness of 200 proves; wrong bits do not.
    let bits200 = [0, 0, 0, 1, 0, 0, 1, 1];
    assert!(proves("range8.zkl", &[200], &bits200));
    assert!(!proves("range8.zkl", &[200], &[1, 1, 1, 1, 1, 1, 1, 1]));

    // A commitment is deterministic in its opening and hides the blinding.
    let c1 = out("commitment.zkl", &[42], &[7]);
    let c2 = out("commitment.zkl", &[42], &[7]);
    assert_eq!(c1, c2);
    assert_ne!(c1, out("commitment.zkl", &[42], &[8]));

    // A nullifier binds the key and index; a MAC binds the key and message.
    assert_ne!(out("nullifier.zkl", &[0], &[99]), out("nullifier.zkl", &[1], &[99]));
    assert_ne!(out("mac.zkl", &[5], &[1]), out("mac.zkl", &[5], &[2]));
}

#[test]
fn hashing_and_membership_programs() {
    // The sponge and a Merkle path both prove and are deterministic.
    assert_eq!(out("sponge2.zkl", &[3, 5], &[]), out("sponge2.zkl", &[3, 5], &[]));
    assert!(proves("merkle3.zkl", &[7, 11, 13, 17, 0, 1, 0], &[]));
    assert!(proves("merkle3.zkl", &[7, 11, 13, 17, 1, 1, 1], &[]));
}
