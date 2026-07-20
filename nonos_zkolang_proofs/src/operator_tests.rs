/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The extended operators: field division, unary negation, and not-equal. Each is
//! front-end sugar over the existing opcodes, so each is proven the same way. The
//! tests check both a true program that verifies and a false claim that has none.

use nonos_zkolang::prove_source_with_inputs;

#[test]
fn division_is_multiplication_by_an_inverse() {
    // 20 / 4 == 5, exact field division.
    let report = prove_source_with_inputs("input a; input b; let q = a / b; output q;", &[20, 4])
        .expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![5]);
}

#[test]
fn division_by_zero_has_no_proof() {
    // Dividing by zero inverts zero, which has no valid trace.
    assert!(
        prove_source_with_inputs("input a; let q = a / 0; output q;", &[7]).is_err(),
        "a division by zero produced a proof"
    );
}

#[test]
fn negation_cancels_addition() {
    // (-x) + x == 0 for any x, so the assertion holds.
    let report = prove_source_with_inputs("input x; assert (-x) + x;", &[9]).expect("run");
    assert!(report.verified, "negation did not cancel");
}

#[test]
fn negation_agrees_with_subtraction_from() {
    // -x + 3 equals 3 - x.
    let report =
        prove_source_with_inputs("input x; assert (-x + 3) - (3 - x);", &[4]).expect("run");
    assert!(report.verified);
}

#[test]
fn not_equal_is_the_complement_of_equal() {
    // 3 != 5 is one.
    let differ = prove_source_with_inputs("input a; input b; let r = a != b; output r;", &[3, 5])
        .expect("run");
    assert_eq!(differ.outputs, vec![1]);
    // 3 != 3 is zero.
    let same = prove_source_with_inputs("input a; input b; let r = a != b; output r;", &[3, 3])
        .expect("run");
    assert_eq!(same.outputs, vec![0]);
}

#[test]
fn logical_not_flips_a_bit() {
    let one = prove_source_with_inputs("input x; output !x;", &[0]).expect("run");
    assert_eq!(one.outputs, vec![1]);
    let zero = prove_source_with_inputs("input x; output !x;", &[1]).expect("run");
    assert_eq!(zero.outputs, vec![0]);
}

#[test]
fn logical_and_is_the_product_of_bits() {
    let t = |a, b| {
        prove_source_with_inputs("input a; input b; output a && b;", &[a, b])
            .expect("run")
            .outputs[0]
    };
    assert_eq!([t(0, 0), t(0, 1), t(1, 0), t(1, 1)], [0, 0, 0, 1]);
}

#[test]
fn logical_or_is_the_bit_union() {
    let t = |a, b| {
        prove_source_with_inputs("input a; input b; output a || b;", &[a, b])
            .expect("run")
            .outputs[0]
    };
    assert_eq!([t(0, 0), t(0, 1), t(1, 0), t(1, 1)], [0, 1, 1, 1]);
}

#[test]
fn boolean_operators_bind_looser_than_comparison() {
    // Parsed as (a == b) || (c == d): equal first pair, unequal second, so one.
    let r = prove_source_with_inputs(
        "input a; input b; input c; input d; output a == b || c == d;",
        &[5, 5, 1, 2],
    )
    .expect("run");
    assert_eq!(r.outputs, vec![1]);
    // And binds tighter than or: false || (true && false) is zero.
    let r2 = prove_source_with_inputs("input a; input b; input c; output a || b && c;", &[0, 1, 0])
        .expect("run");
    assert_eq!(r2.outputs, vec![0]);
}

#[test]
fn the_cypherpunk_keyword_spelling_is_the_same_language() {
    // public/witness/reveal/prove are aliases of input/secret/output/assert. A program
    // written in either spelling compiles to the same proof.
    let plain = prove_source_with_inputs("input a; secret b; output a + b; assert a - a;", &[3, 4])
        .expect("run");
    let styled =
        prove_source_with_inputs("public a; witness b; reveal a + b; prove a - a;", &[3, 4])
            .expect("run");
    assert!(plain.verified && styled.verified);
    assert_eq!(plain.outputs, styled.outputs);
    assert_eq!(styled.outputs, vec![7]);
}
