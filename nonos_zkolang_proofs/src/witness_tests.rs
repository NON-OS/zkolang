/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The private witness. A `secret` input feeds the run but never enters the
//! public statement, so a proof attests knowledge of a hidden value satisfying a
//! public relation. This is a private witness, not full zero-knowledge: the STARK
//! is not hiding, so the openings could leak trace values. What is proven here is
//! that the witness is not part of the committed public statement.

use nonos_zkolang::{prove_source_with_witness, RunError};

#[test]
fn a_secret_square_root_is_proven_without_revealing_it() {
    // Prove knowledge of w with w * w == 25, keeping w private. The public
    // statement carries no input; only that the relation holds.
    let src = "secret w; assert w * w - 25;";
    let report = prove_source_with_witness(src, &[], &[5]).expect("run");
    assert!(report.verified, "a valid secret witness was rejected");
    assert!(
        report.outputs.is_empty(),
        "the witness leaked into the outputs"
    );
}

#[test]
fn a_wrong_secret_witness_has_no_proof() {
    // w = 6 does not satisfy w * w == 25, so the assertion fails and there is no
    // proof, without revealing anything.
    let src = "secret w; assert w * w - 25;";
    assert!(
        prove_source_with_witness(src, &[], &[6]).is_err(),
        "a wrong witness produced a proof"
    );
}

#[test]
fn a_secret_witness_drives_a_public_output() {
    // A mix: a public input and a private witness produce a public output. Only
    // the public input and the output are in the statement.
    let src = "input a; secret w; let y = a * w; output y;";
    let report = prove_source_with_witness(src, &[3], &[7]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![21], "the output was wrong");
}

#[test]
fn inputs_must_match_what_the_program_declares() {
    // Two public inputs declared, one supplied and a secret alongside: the second `input`
    // would read the secret, a value the public statement does not carry, and the proof
    // would claim a public input nobody bound. The run is refused instead.
    let src = "input a; input b; output a + b;";
    let short = prove_source_with_witness(src, &[1], &[2]);
    assert_eq!(
        short,
        Err(RunError::InputCount {
            public_expected: 2,
            public_got: 1,
            secret_expected: 0,
            secret_got: 1,
        })
    );
    // A declared secret passed as a public input would put it in the statement.
    let revealed = prove_source_with_witness("input a; secret w; output a;", &[1, 2], &[]);
    assert!(matches!(revealed, Err(RunError::InputCount { .. })));
    // Too many secrets would misplace the comparison advice.
    let extra = prove_source_with_witness("secret w; output w < 5;", &[], &[1, 2]);
    assert!(matches!(extra, Err(RunError::InputCount { .. })));
    // The exact counts prove.
    let exact = prove_source_with_witness(src, &[1, 2], &[]).expect("run");
    assert!(exact.verified);
    assert_eq!(exact.outputs, vec![3]);
}
