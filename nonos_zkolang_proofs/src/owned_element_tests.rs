/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A call that returns an element of an array built for it gives that register back. The
 * result was never marked temporary, so every such call held one register for the rest
 * of the program, and forty calls in a loop ran out of registers.
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
fn an_element_of_an_owned_argument_is_freed() {
    let src = "fn first(v) = v[0];\ninput x;\nfor i in 0..40 {\n  output first([x + i, 2]);\n}";
    let want: Vec<u64> = (1..41).collect();
    assert_eq!(outputs(src, &[1]), want);
    /* A binding the caller still holds keeps its register when it is the element returned. */
    let held =
        "fn first(v) = v[0];\ninput x;\nlet y = x * 3;\noutput first([y, 2]);\noutput y + 1;";
    assert_eq!(outputs(held, &[2]), vec![6, 7]);
}
