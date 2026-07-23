/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Array parameters. A function can take a whole array as one argument and index it inside,
//! so an operation over a vector is named once and reused. Each program compiles and proves,
//! an array threads through nested calls, an array and a scalar parameter mix, and indexing a
//! parameter that was given a scalar is the error it should be.

use nonos_zkolang::{compile_source, prove_source_with_inputs};

#[test]
fn a_function_takes_a_vector() {
    let src = "\
fn dot3(a, b) = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
fn sum3(v) = v[0] + v[1] + v[2];
input a0;
input a1;
input a2;
input b0;
input b1;
input b2;
let u = [a0, a1, a2];
let w = [b0, b1, b2];
output dot3(u, w);
output sum3(u);";
    let report = prove_source_with_inputs(src, &[1, 2, 3, 4, 5, 6]).expect("run");
    assert!(report.verified, "a function over a vector was rejected");
    assert_eq!(report.outputs, vec![32, 6], "dot then sum");
}

#[test]
fn a_vector_and_a_scalar_parameter_mix() {
    let src = "\
fn wsum(v, k) = (v[0] + v[1] + v[2]) * k;
input a;
input b;
input c;
input k;
let u = [a, b, c];
output wsum(u, k);";
    let report = prove_source_with_inputs(src, &[1, 2, 3, 10]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![60], "(1 + 2 + 3) * 10");
}

#[test]
fn a_vector_threads_through_nested_calls() {
    // An array argument passes into a function that passes it to another.
    let src = "\
fn head(v) = v[0];
fn head_plus(v, k) = head(v) + k;
input a;
input b;
input k;
let u = [a, b];
output head_plus(u, k);";
    let report = prove_source_with_inputs(src, &[7, 9, 5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![12], "7 + 5");
}

#[test]
fn indexing_a_scalar_parameter_is_an_error() {
    // A parameter given a scalar has no array to index, so `v[0]` inside is a type error.
    let src = "fn bad(v) = v[0];\ninput a;\noutput bad(a);";
    assert!(
        compile_source(src).is_err(),
        "indexing a scalar parameter compiled"
    );
}
