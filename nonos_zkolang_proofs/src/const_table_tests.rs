/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Constant tables, read by a compile-time index. A table names a fixed list of
//! field values once; a read with a static index folds to one entry, so the table
//! never reaches the trace. This is the data shape a hash needs: round constants and
//! the mixing matrix become `RC[r * width + i]` instead of hundreds of literals.

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError};

#[test]
fn a_literal_index_reads_the_entry() {
    let src = "const T = [10, 20, 30]; output T[1];";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![20]);
}

#[test]
fn a_loop_variable_indexes_a_table() {
    // Sum the whole table by indexing it with the loop variable, the shape a round
    // schedule uses.
    let src = "const W = [1, 2, 3, 4];
               let acc = 0;
               for i in 0 .. 4 { let acc = acc + W[i]; }
               output acc;";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![10], "1 + 2 + 3 + 4");
}

#[test]
fn an_index_expression_folds_at_compile_time() {
    // A two-dimensional layout addressed as row * width + col, exactly how a hash
    // lays out its round constants. The row is a loop variable, so `r * 3 + col`
    // folds to a static offset at each unrolled step.
    let src = "const RC = [0, 1, 2, 3, 4, 5];
               for r in 1 .. 2 { output RC[r * 3 + 2]; }";
    let report = prove_source_with_inputs(src, &[]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![5], "row 1, col 2 of a width-3 table");
}

#[test]
fn a_table_value_enters_arithmetic() {
    let src = "const K = [7, 11]; input x; output x * K[0] + K[1];";
    let report = prove_source_with_inputs(src, &[3]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![32], "3 * 7 + 11");
}

#[test]
fn an_out_of_range_index_is_a_compile_error() {
    let src = "const T = [1, 2]; output T[2];";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::IndexOutOfBounds)
    ));
}

#[test]
fn indexing_an_unknown_table_is_an_error() {
    assert!(matches!(
        compile_source("output NOPE[0];"),
        Err(CompileError::UnknownConst { .. })
    ));
}

#[test]
fn a_runtime_index_is_rejected() {
    // The index must be static; a public input cannot address a table, because a
    // data-dependent index would break the straight-line shape.
    let src = "const T = [1, 2, 3]; input i; output T[i];";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::NonConstantIndex)
    ));
}

#[test]
fn indexing_a_non_table_is_an_error() {
    let src = "let x = 5; output x[0];";
    assert!(matches!(
        compile_source(src),
        Err(CompileError::NotIndexable)
    ));
}

#[test]
fn a_scalar_constant_reads_by_name() {
    let src = "const N = 5; input x; output x + N;";
    let report = prove_source_with_inputs(src, &[3]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![8]);
}

#[test]
fn scalar_and_table_constants_coexist() {
    let src = "const K = 10; const T = [1, 2, 3]; input x; output x * K + T[1];";
    let report = prove_source_with_inputs(src, &[2]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![22], "2*10 + 2");
}

#[test]
fn indexing_a_scalar_is_a_type_error() {
    assert!(matches!(
        compile_source("const N = 5; output N[0];"),
        Err(CompileError::NotIndexable)
    ));
}
