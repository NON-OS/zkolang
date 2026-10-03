/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An assertion keeps its meaning through constant folding: `assert a == b` states the
 * relation, and folding two constants decides it rather than inverting it.
 */

use nonos_zkolang::{compile_source_unoptimized, evaluate, prove_source_with_inputs};

/** Whether a program has a proof, optimized, and whether the unoptimized form agrees. */
fn proves(src: &str) -> bool {
    let optimized = prove_source_with_inputs(src, &[]).is_ok_and(|r| r.verified);
    let plain = compile_source_unoptimized(src).expect("compile");
    let unoptimized = evaluate(&plain, &[], &[]).is_ok();
    assert_eq!(
        optimized, unoptimized,
        "the optimizer changed what `{src}` proves"
    );
    optimized
}

#[test]
fn an_assertion_between_constants_keeps_its_meaning() {
    /*
     * `assert a == b` is an equality, while any other asserted expression is required to be
     * zero. Folding the equality of two constants to its bit and then asserting that bit
     * zero turned every true constant equality into a false statement and every false one
     * into a true one.
     */
    assert!(proves("assert 3 == 3; output 1;"));
    assert!(!proves("assert 3 == 4; output 1;"));
    assert!(proves("assert 3 != 4; output 1;"));
    assert!(!proves("assert 3 != 3; output 1;"));
    /* The same through constant propagation of a binding. */
    assert!(!proves("let k = 5; assert k == 6; output 1;"));
    assert!(proves("let k = 5; assert k == 5; output 1;"));
    assert!(!proves(
        "for i in 0..2 { let k = 5; assert k != 5; } output 1;"
    ));
}
