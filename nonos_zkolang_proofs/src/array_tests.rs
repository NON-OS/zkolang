/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! First-class arrays: fixed-size vectors of values, read by a compile-time index.
//! An array holds computed values, not only literals, and a loop can rebuild one
//! each iteration and still fit the register file, because a shadowed array's
//! registers are reclaimed. These are the data shape a state vector needs.

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError};

#[test]
fn an_array_holds_and_reads_values() {
    let src = "let v = [10, 20, 30]; output v[1];";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![20]);
}

#[test]
fn array_elements_are_computed_expressions() {
    let src = "input x; let v = [x, x * x, x * x * x]; output v[2];";
    let report = prove_source_with_inputs(src, &[3]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![27], "the cubed element");
}

#[test]
fn a_loop_variable_indexes_an_array() {
    // Sum the elements by indexing with the loop variable.
    let src = "input a; input b; input c; input d;
               let v = [a, b, c, d];
               let acc = 0;
               for i in 0 .. 4 { let acc = acc + v[i]; }
               output acc;";
    let report = prove_source_with_inputs(src, &[1, 2, 3, 4]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![10]);
}

#[test]
fn an_array_is_rebuilt_each_iteration_and_fits() {
    // A rolling state vector: each round shifts and folds. Rebinding reclaims the old
    // array's registers, so this does not exhaust the file.
    let src = "let s = [1, 1, 1];
               for r in 0 .. 8 {
                   let s = [s[0] + s[1], s[1] + s[2], s[2] + s[0]];
               }
               output s[0];";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
}

#[test]
fn a_whole_array_is_not_a_value() {
    // Using an array name where a single value is required is a type error.
    let src = "let v = [1, 2]; output v;";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::ArrayNotScalar)
    ));
}

#[test]
fn an_array_literal_is_not_a_value() {
    let src = "output [1, 2, 3];";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::ArrayNotScalar)
    ));
}

#[test]
fn an_out_of_range_array_index_is_an_error() {
    let src = "let v = [1, 2, 3]; output v[3];";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::IndexOutOfBounds { .. })
    ));
}

#[test]
fn a_runtime_array_index_is_rejected() {
    let src = "input i; let v = [1, 2, 3]; output v[i];";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::NonConstantIndex)
    ));
}
