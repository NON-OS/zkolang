/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The optimizer, checked. Constant sub-expressions fold to one value and no-op
//! arithmetic is removed, so the trace shrinks, and the outputs are unchanged, so the
//! fold is behaviour preserving.

use nonos_zkolang::{
    compile_source, compile_source_unoptimized, evaluate, prove_source_with_inputs,
};

#[test]
fn constants_fold_to_a_single_value() {
    // 2 + 3 * 4 folds to 14: one immediate, one output, one halt.
    let ops = compile_source("output 2 + 3 * 4;").expect("compile");
    assert_eq!(ops.len(), 3, "constant program did not fold to one value");
    let r = prove_source_with_inputs("output 2 + 3 * 4;", &[]).expect("run");
    assert_eq!(r.outputs, vec![14]);
}

#[test]
fn algebraic_identities_are_removed() {
    // x * 1 + 0 - 0 is x: one input read, one output, one halt.
    let ops = compile_source("input x; output x * 1 + 0 - 0;").expect("compile");
    assert_eq!(ops.len(), 3, "identities were not removed");
    let r = prove_source_with_inputs("input x; output x * 1 + 0 - 0;", &[7]).expect("run");
    assert_eq!(r.outputs, vec![7]);
}

#[test]
fn folding_preserves_behavior() {
    // The mixed constant and variable form computes the same value the source means.
    let r = prove_source_with_inputs("input x; output (x + 0) * (2 + 3);", &[4]).expect("run");
    assert_eq!(r.outputs, vec![20]);
    // A real circuit is unchanged: the cube of nine is still seven hundred twenty nine.
    let c = prove_source_with_inputs("input x; let y = x * x * x; output y;", &[9]).expect("run");
    assert_eq!(c.outputs, vec![729]);
}

#[test]
fn constant_propagation_raises_the_register_ceiling() {
    // Forty constant bindings summed would need forty live registers, past the file of
    // thirty two, so without propagation this is TooManyRegisters. Propagation inlines
    // them and the program folds to a single value.
    let mut src = String::new();
    for i in 0..40u64 {
        src.push_str(&format!("let c{i} = {};\n", i + 1));
    }
    src.push_str("output ");
    for i in 0..40u64 {
        if i > 0 {
            src.push_str(" + ");
        }
        src.push_str(&format!("c{i}"));
    }
    src.push(';');
    let ops = compile_source(&src).expect("compile");
    assert!(
        ops.len() < 8,
        "constants were not propagated: {} ops",
        ops.len()
    );
    assert_eq!(
        prove_source_with_inputs(&src, &[]).unwrap().outputs,
        vec![820]
    );
}

#[test]
fn propagation_preserves_shadowing_and_runtime_values() {
    // The first k is a constant and inlines; the second rebinds k to a runtime value and
    // shadows it, so the output is the runtime value, not the constant.
    let r = prove_source_with_inputs("let k = 5; input x; let k = x + 1; output k;", &[9])
        .expect("run");
    assert_eq!(r.outputs, vec![10]);
}

#[test]
fn common_subexpressions_are_shared() {
    // (x + 1) * (x + 1) computes x + 1 once, so it is one add, not two.
    let ops = compile_source("input x; output (x + 1) * (x + 1);").expect("compile");
    assert!(
        ops.len() <= 6,
        "subexpression not shared: {} ops",
        ops.len()
    );
    let r = prove_source_with_inputs("input x; output (x + 1) * (x + 1);", &[4]).expect("run");
    assert_eq!(r.outputs, vec![25]);
}

#[test]
fn cse_shares_the_largest_repeat_and_stays_correct() {
    // The whole product (a + b) * c is shared where it repeats, not only a + b.
    let r = prove_source_with_inputs(
        "input a; input b; input c; output ((a + b) * c) + ((a + b) * c);",
        &[2, 3, 4],
    )
    .expect("run");
    assert_eq!(r.outputs, vec![40]);
}

// Whether a program has a proof, optimized, and whether the unoptimized form of it agrees.
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
    // `assert a == b` is an equality, while any other asserted expression is required to be
    // zero. Folding the equality of two constants to its bit and then asserting that bit
    // zero turned every true constant equality into a false statement and every false one
    // into a true one.
    assert!(proves("assert 3 == 3; output 1;"));
    assert!(!proves("assert 3 == 4; output 1;"));
    assert!(proves("assert 3 != 4; output 1;"));
    assert!(!proves("assert 3 != 3; output 1;"));
    // The same through constant propagation of a binding.
    assert!(!proves("let k = 5; assert k == 6; output 1;"));
    assert!(proves("let k = 5; assert k == 5; output 1;"));
    assert!(!proves(
        "for i in 0..2 { let k = 5; assert k != 5; } output 1;"
    ));
}

// Whether a program has a proof on the given public inputs, optimized and unoptimized alike.
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
    // Multiplying by zero, or selecting on a constant, discards an operand's value but not
    // the constraints evaluating it carries: the inverse of zero, a select on a condition
    // that is not a bit. Both arms of a select are evaluated in this edition, so the arm a
    // constant condition does not take still constrains.
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
    // With nothing to lose the identities still apply.
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
