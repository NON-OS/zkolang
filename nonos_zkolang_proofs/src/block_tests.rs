/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Function bodies with local bindings. A function is no longer one expression: it can
//! open a block, name intermediate steps, and return a result, the way real code reads.
//! Each program here is compiled and proven, so the block lowers to a trace the STARK
//! accepts, and its scope, ordering, and shadowing all hold under the proof.

use nonos_zkolang::{compile_source, prove_source_with_inputs};

#[test]
fn a_block_must_yield_a_result() {
    // A block with bindings but no final expression has no value, so it is rejected, not
    // read as returning nothing. An empty block is rejected the same way.
    assert!(
        compile_source("fn f(x) { let a = x + 1; }\ninput y;\noutput f(y);").is_err(),
        "a result-less block compiled"
    );
    assert!(
        compile_source("input y;\nlet z = { };\noutput z;").is_err(),
        "an empty block compiled"
    );
}

#[test]
fn a_local_does_not_escape_its_block() {
    // A name bound inside a block is gone once the block ends, so reading it afterward is
    // an unknown name, not a stale value. This is the scope the compiler tears down.
    assert!(
        compile_source("input y;\nlet z = { let a = y + 1; a };\noutput a;").is_err(),
        "a block-local name leaked to the outer scope"
    );
}

#[test]
fn a_function_body_of_local_steps() {
    // Name the square and the cube, return their sum. At three: 9 + 27 = 36.
    let src = "\
fn poly(x) {
    let sq = x * x;
    let cube = sq * x;
    return sq + cube;
}
input x;
output poly(x);";
    let report = prove_source_with_inputs(src, &[3]).expect("run");
    assert!(report.verified, "a block-bodied function was rejected");
    assert_eq!(report.outputs, vec![36]);
}

#[test]
fn a_trailing_expression_is_the_result() {
    // No return keyword: the last expression is the value. s = a + b, then s * s.
    let src = "\
fn sq_of_sum(a, b) {
    let s = a + b;
    s * s
}
input x;
output sq_of_sum(x, x);";
    let report = prove_source_with_inputs(src, &[5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![100], "ten squared");
}

#[test]
fn a_block_is_an_expression_anywhere() {
    // A block used inline as a value, not only as a function body.
    let src = "\
input x;
let y = { let t = x + 1; t * t };
output y;";
    let report = prove_source_with_inputs(src, &[4]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![25], "five squared");
}

#[test]
fn a_local_shadows_its_outer_name() {
    // Two lets reuse the name x; each sees the previous binding, the param first.
    let src = "\
fn step(x) {
    let x = x + 1;
    let x = x * 2;
    return x;
}
input v;
output step(v);";
    let report = prove_source_with_inputs(src, &[3]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![8], "(3 + 1) * 2");
}

#[test]
fn blocks_call_other_functions_and_compose() {
    // A block body calls another function and threads its result through more locals,
    // the shape real library code takes.
    let src = "\
fn double(n) = n + n;
fn build(a, b) {
    let u = double(a);
    let v = double(b);
    let w = u * v;
    return w + a;
}
input x;
output build(x, x);";
    let report = prove_source_with_inputs(src, &[3]).expect("run");
    assert!(report.verified);
    // double(3) = 6, 6 * 6 = 36, + 3 = 39.
    assert_eq!(report.outputs, vec![39]);
}
