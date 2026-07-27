/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Array returns. A function can build and return a whole vector, so an elementwise
//! operation is written once and its result named and indexed. Each program compiles and
//! proves: a function returns a vector, two returned vectors compose through a binding, a
//! block body returns a vector, and a plain binding aliases an existing array.

use nonos_zkolang::prove_source_with_inputs;

#[test]
fn a_function_returns_a_vector() {
    let src = "\
fn scale3(v, k) = [v[0] * k, v[1] * k, v[2] * k];
input a;
input b;
input c;
input k;
let u = [a, b, c];
let s = scale3(u, k);
output s[0];
output s[1];
output s[2];";
    let report = prove_source_with_inputs(src, &[1, 2, 3, 10]).expect("run");
    assert!(report.verified, "a vector-returning function was rejected");
    assert_eq!(report.outputs, vec![10, 20, 30]);
}

#[test]
fn returned_vectors_compose_through_a_binding() {
    let src = "\
fn scale3(v, k) = [v[0] * k, v[1] * k, v[2] * k];
fn add3(a, b) = [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
input a;
input b;
input c;
let u = [a, b, c];
let d = scale3(u, 2);
let s = add3(u, d);
output s[0];
output s[1];
output s[2];";
    let report = prove_source_with_inputs(src, &[1, 2, 3]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![3, 6, 9], "u + 2u");
}

#[test]
fn a_block_body_returns_a_vector() {
    let src = "\
fn twice(v) {
    let k = 2;
    [v[0] * k, v[1] * k]
}
input a;
input b;
let u = [a, b];
let w = twice(u);
output w[0];
output w[1];";
    let report = prove_source_with_inputs(src, &[5, 7]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![10, 14]);
}

#[test]
fn a_binding_aliases_an_array() {
    let src = "input a;\ninput b;\nlet u = [a, b];\nlet w = u;\noutput w[0];\noutput w[1];";
    let report = prove_source_with_inputs(src, &[3, 4]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![3, 4]);
}

#[test]
fn an_array_argument_need_not_be_named_first() {
    // A returned vector and an array literal pass directly as arguments, not only named ones.
    let src = "\
fn scale3(v, k) = [v[0] * k, v[1] * k, v[2] * k];
fn add3(a, b) = [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
fn dot3(a, b) = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
input x;
input y;
input z;
let u = [x, y, z];
let s = add3(scale3(u, 2), u);
output s[0];
output s[1];
output s[2];
output dot3([1, 2, 3], u);";
    let report = prove_source_with_inputs(src, &[1, 2, 3]).expect("run");
    assert!(report.verified);
    assert_eq!(
        report.outputs,
        vec![3, 6, 9, 14],
        "2u + u elementwise, then dot"
    );
}
