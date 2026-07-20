/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The optimizer, checked. Constant sub-expressions fold to one value and no-op
//! arithmetic is removed, so the trace shrinks, and the outputs are unchanged, so the
//! fold is behaviour preserving.

use nonos_zkolang::{compile_source, prove_source_with_inputs};

#[test]
fn constants_fold_to_a_single_value() {
    // 2 + 3 * 4 folds to 14: one immediate, one output, one halt.
    let ops = compile_source("output 2 + 3 * 4;").expect("compile");
    assert_eq!(ops.len(), 3, "constant program did not fold to one value");
    let r = prove_source_with_inputs("output 2 + 3 * 4;", &[]).expect("run");
    assert_eq!(r.outputs, vec![14]);
}

#[test]
fn algebraic_identities_are_removed() {
    // x * 1 + 0 - 0 is x: one input read, one output, one halt.
    let ops = compile_source("input x; output x * 1 + 0 - 0;").expect("compile");
    assert_eq!(ops.len(), 3, "identities were not removed");
    let r = prove_source_with_inputs("input x; output x * 1 + 0 - 0;", &[7]).expect("run");
    assert_eq!(r.outputs, vec![7]);
}

#[test]
fn folding_preserves_behavior() {
    // The mixed constant and variable form computes the same value the source means.
    let r = prove_source_with_inputs("input x; output (x + 0) * (2 + 3);", &[4]).expect("run");
    assert_eq!(r.outputs, vec![20]);
    // A real circuit is unchanged: the cube of nine is still seven hundred twenty nine.
    let c = prove_source_with_inputs("input x; let y = x * x * x; output y;", &[9]).expect("run");
    assert_eq!(c.outputs, vec![729]);
}
