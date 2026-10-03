/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Folding never discards an operand that carries a constraint. */

use nonos_zkolang::{
    compile_source, compile_source_unoptimized, evaluate, prove_source_with_inputs,
};

/** Whether a program proves on these public inputs, optimized and unoptimized alike. */
fn proves_on(src: &str, public: &[u64]) -> bool {
    let optimized = prove_source_with_inputs(src, public).is_ok_and(|r| r.verified);
    let plain = compile_source_unoptimized(src).expect("compile");
    let unoptimized = evaluate(&plain, public, &[]).is_ok();
    assert_eq!(
        optimized, unoptimized,
        "the optimizer changed what `{src}` proves"
    );
    optimized
}

#[test]
fn folding_never_drops_a_constraint() {
    /*
     * Multiplying by zero, or selecting on a constant, discards an operand's value but not
     * the constraints evaluating it carries: the inverse of zero, a select on a condition
     * that is not a bit. Both arms of a select are evaluated in this edition, so the arm a
     * constant condition does not take still constrains.
     */
    assert!(!proves_on("input x; assert inv(x) * 0; output 1;", &[0]));
    assert!(!proves_on("input x; assert 0 * inv(x); output 1;", &[0]));
    assert!(!proves_on("input x; output sel(x, 1, 2) * 0;", &[2]));
    assert!(!proves_on(
        "input x; output if 1 { 5 } else { inv(x) };",
        &[0]
    ));
    assert!(!proves_on(
        "input x; output if 0 { 1 / x } else { 5 };",
        &[0]
    ));
    /* With nothing to lose the identities still apply. */
    assert!(proves_on(
        "input x; output x * 0 + if 1 { x } else { 2 };",
        &[3]
    ));
    let ops = compile_source("input x; output x * 0;").expect("compile");
    assert_eq!(
        ops.len(),
        4,
        "a constraint-free operand should still fold away"
    );
}
