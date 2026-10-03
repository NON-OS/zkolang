/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A `_` slot of a destructuring binds nothing in either build. Constant propagation took
 * it as a binding, so it hid an earlier `let _ = 5;` from substitution, the optimizer
 * dropped that `let`, and a later read of `_` named nothing only in the optimized build.
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate};

/** The outputs of both builds, which must agree. */
fn outputs(src: &str, public: &[u64]) -> Vec<u64> {
    let opt = evaluate(&compile_source(src).expect("compile"), public, &[]).expect("run");
    let plain = compile_source_unoptimized(src).expect("compile");
    let unopt = evaluate(&plain, public, &[]).expect("run");
    assert_eq!(opt, unopt, "the builds disagree on `{src}`");
    opt
}

#[test]
fn a_wildcard_slot_hides_nothing() {
    let top = "input x;\nlet _ = 5;\nlet (_, b) = (x, x);\noutput _ + b;";
    assert_eq!(outputs(top, &[7]), vec![12]);
    let block = "input x;\nlet _ = 5;\noutput { let (_, b) = (x, x); _ + b };";
    assert_eq!(outputs(block, &[7]), vec![12]);
    let looped = "input x;\nlet _ = 5;\nfor i in 0..2 { let (_, b) = (x, i); output _ + b; }";
    assert_eq!(outputs(looped, &[7]), vec![5, 6]);
}
