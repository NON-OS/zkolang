/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Folding an asserted expression never changes which relation the assertion states.
 * `assert e` requires `e` to be zero unless `e` is written as `==` or `!=`; folding
 * `(a == b) + 0` to `a == b` turned "the equality bit is zero" into "a equals b".
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate};

/** Whether both builds accept these public inputs, which must agree. */
fn accepts(src: &str, public: &[u64]) -> bool {
    let opt = evaluate(&compile_source(src).expect("compile"), public, &[]).is_ok();
    let plain = compile_source_unoptimized(src).expect("compile");
    let unopt = evaluate(&plain, public, &[]).is_ok();
    assert_eq!(opt, unopt, "the builds disagree on `{src}` for {public:?}");
    opt
}

#[test]
fn a_folded_assertion_keeps_its_relation() {
    let eq_bit = "input a;\ninput b;\nassert (a == b) + 0;\noutput 1;";
    assert!(!accepts(eq_bit, &[3, 3]));
    assert!(accepts(eq_bit, &[3, 4]));
    let ne_bit = "input a;\ninput b;\nassert (a != b) * 1;\noutput 1;";
    assert!(!accepts(ne_bit, &[3, 4]));
    assert!(accepts(ne_bit, &[3, 3]));
    let selected = "input a;\ninput b;\nassert if 1 { a == b } else { 7 };\noutput 1;";
    assert!(!accepts(selected, &[5, 5]));
    assert!(accepts(selected, &[5, 6]));
}
