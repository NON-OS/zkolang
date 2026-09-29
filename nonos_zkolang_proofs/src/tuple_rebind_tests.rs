/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A destructuring `let` that rebinds names it also reads: a swap, a Fibonacci step, a
 * value and its predecessor. The name's old register can still be carried by a later slot
 * of the same tuple, so it must not return to the pool before that slot is bound.
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
fn a_rebinding_tuple_keeps_the_registers_its_later_slots_carry() {
    let swap = "input x;\ninput y;\nlet (x, y) = (y, x);\nlet z = x + 100;\noutput y;\noutput z;";
    assert_eq!(outputs(swap, &[3, 5]), vec![3, 105]);
    let fib = "input a;\ninput b;\nfor i in 0..5 { let (a, b) = (a + b, a); }\n\
               let s = a * 100;\nlet t = a * 1000;\noutput a;\noutput b;\noutput s;\noutput t;";
    assert_eq!(outputs(fib, &[1, 1]), vec![13, 8, 1300, 13000]);
    let step = "fn step(p, q) = (p + q, p);\ninput a;\ninput b;\nlet (a, b) = step(a, b);\n\
                let k = a * 7;\nlet m = a * 9;\noutput a;\noutput b;\noutput k;\noutput m;";
    assert_eq!(outputs(step, &[2, 3]), vec![5, 2, 35, 45]);
    let from_array = "input x;\ninput y;\nlet a = [x + 1, y + 2];\nlet (a, b) = (a[0], a[1]);\n\
                      let k = x * 1000;\noutput a;\noutput b;\noutput k;";
    assert_eq!(outputs(from_array, &[3, 5]), vec![4, 7, 3000]);
    let old_new = "input x;\nlet (x, old) = (x + 1, x);\nlet k = 77;\noutput x + k;\noutput old;";
    assert_eq!(outputs(old_new, &[10]), vec![88, 10]);
}
