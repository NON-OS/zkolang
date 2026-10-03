/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Constant propagation reaches into blocks. A top-level `let` that folds to a constant is
 * dropped and its uses inlined; a block that was left out of the substitution then read
 * an older binding of the name, or no binding at all.
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
fn a_block_sees_a_propagated_constant() {
    /* The block read the input instead of the constant that shadows it. */
    assert_eq!(
        outputs("input x;\nlet x = 5;\noutput { let y = x; y };", &[9]),
        vec![5]
    );
    /* The block named a binding the optimizer had dropped. */
    assert_eq!(outputs("let k = 5;\noutput { k + 1 };", &[]), vec![6]);
    /* A function argument that is a block. */
    assert_eq!(
        outputs("fn f(a) = a * 2;\nlet k = 4;\noutput f({ k + 1 });", &[]),
        vec![10]
    );
    /* A block's own binding shadows the outer constant from there on. */
    let shadow = "input x;\nlet k = 5;\noutput { let a = k; let k = x; a + k };";
    assert_eq!(outputs(shadow, &[7]), vec![12]);
}
