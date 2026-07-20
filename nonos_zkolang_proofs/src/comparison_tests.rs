/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Ordered comparison, proven. The operators auto-witness their bit decomposition, so a
//! program just writes `a < b`. The accept tests check the operators compute the right
//! bit; the reject tests are the soundness gate: a false ordering has no proof, and an
//! operand outside the sixteen-bit range has no proof, because the range proofs pin the
//! operands and the composition pins the difference bits.

use nonos_zkolang::prove_source_with_inputs;

fn out(src: &str, inputs: &[u64]) -> u64 {
    let r = prove_source_with_inputs(src, inputs).expect("run");
    assert!(r.verified);
    r.outputs[0]
}

#[test]
fn less_than_computes_the_order() {
    assert_eq!(out("input a; input b; output a < b;", &[3, 10]), 1);
    assert_eq!(out("input a; input b; output a < b;", &[10, 3]), 0);
    assert_eq!(out("input a; input b; output a < b;", &[5, 5]), 0);
    assert_eq!(out("input a; input b; output a < b;", &[0, 65535]), 1);
}

#[test]
fn the_four_ordered_operators() {
    assert_eq!(out("input a; input b; output a <= b;", &[5, 5]), 1);
    assert_eq!(out("input a; input b; output a <= b;", &[6, 5]), 0);
    assert_eq!(out("input a; input b; output a > b;", &[10, 3]), 1);
    assert_eq!(out("input a; input b; output a > b;", &[3, 10]), 0);
    assert_eq!(out("input a; input b; output a >= b;", &[3, 10]), 0);
    assert_eq!(out("input a; input b; output a >= b;", &[10, 10]), 1);
}

#[test]
fn comparison_composes_with_boolean_operators() {
    // In range on both sides: 4 < 7 and 7 < 9, so one.
    let src = "input x; output (3 < x) && (x < 9);";
    assert_eq!(out(src, &[7]), 1);
    assert_eq!(out(src, &[9]), 0);
}

#[test]
fn a_false_ordering_has_no_proof() {
    // assert (a < b) - 1 forces a < b; with a >= b there is no witness.
    assert!(
        prove_source_with_inputs("input a; input b; assert (a < b) - 1;", &[10, 3]).is_err(),
        "10 < 3 was provable"
    );
    assert!(
        prove_source_with_inputs("input a; input b; assert (a < b) - 1;", &[5, 5]).is_err(),
        "5 < 5 was provable"
    );
}

#[test]
fn an_out_of_range_operand_has_no_proof() {
    // The range proof pins the operand to sixteen bits, so a larger value cannot compare.
    assert!(
        prove_source_with_inputs("input a; output a < 5;", &[70_000]).is_err(),
        "an operand above the sixteen-bit range compared"
    );
}
