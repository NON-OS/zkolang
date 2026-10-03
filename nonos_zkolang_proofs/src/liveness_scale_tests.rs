/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Liveness costs time in proportion to the program. The table behind it held, for every
 * statement, a copy of every name read after it, so a chain of sixty thousand `let`s took
 * minutes before lowering reported anything; it is now one last-read index per name.
 */

use std::time::{Duration, Instant};

use nonos_zkolang::{compile_source, evaluate};

#[test]
fn a_long_chain_of_lets_compiles_quickly() {
    let n = 60_000;
    let mut src = String::from("input x;\nlet a0 = x;\n");
    for i in 1..n {
        src.push_str(&format!("let a{i} = a{} + 1;\n", i - 1));
    }
    src.push_str(&format!("output a{};", n - 1));
    let start = Instant::now();
    let ops = compile_source(&src).expect("compile");
    assert!(
        start.elapsed() < Duration::from_secs(60),
        "{:?}",
        start.elapsed()
    );
    assert_eq!(evaluate(&ops, &[5], &[]).expect("run"), vec![5 + (n - 1)]);
}
