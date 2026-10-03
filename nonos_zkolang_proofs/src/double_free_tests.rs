/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One register never enters the free list twice. An array can hold one register in
 * several slots, from source or from common-subexpression elimination, and retiring it
 * returned that register once per slot, so two later values shared it.
 */

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate};

/** The outputs of both builds on one public input, which must agree. */
fn outputs(src: &str, x: u64) -> Vec<u64> {
    let opt = evaluate(&compile_source(src).expect("compile"), &[x], &[]).expect("run");
    let plain = compile_source_unoptimized(src).expect("compile");
    let unopt = evaluate(&plain, &[x], &[]).expect("run");
    assert_eq!(opt, unopt, "the builds disagree on `{src}`");
    opt
}

#[test]
fn retiring_an_array_with_a_repeated_register_frees_it_once() {
    /* CSE turns `[x * x, x * x]` into two slots of one register. */
    let cse = "input x;\nlet w = [x * x, x * x];\noutput w[0] + w[1];\nlet w = [x + 5];\n\
               let p = x * 2;\nlet q = x * 3;\noutput p;\noutput q;\noutput w[0];";
    assert_eq!(outputs(cse, 10), vec![200, 20, 30, 15]);
    /* A literal repeating a binding, retired by a scalar rebinding. */
    let scalar = "input x;\nlet y = x + 1;\nlet a = [y, y];\noutput a[0];\nlet a = x * 5;\n\
                  let p = x * 2;\nlet q = x * 3;\noutput p;\noutput q;\noutput a;";
    assert_eq!(outputs(scalar, 10), vec![11, 20, 30, 50]);
    /* A function returning one value twice, rebound in a loop. */
    let splat = "fn splat(a) = [a, a];\ninput x;\n\
                 for i in 0..2 { let w = splat(x + i); output w[0] + w[1]; }\n\
                 let p = x * 2;\nlet q = x * 3;\noutput p;\noutput q;";
    assert_eq!(outputs(splat, 10), vec![20, 22, 20, 30]);
}
