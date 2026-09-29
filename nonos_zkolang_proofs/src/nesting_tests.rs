/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Deeply nested input is refused with an error, never by overflowing the stack. */

use crate::small_stack::compiles_on_a_small_stack;
use nonos_zkolang::CompileError;

#[test]
fn deep_nesting_is_an_error_not_a_stack_overflow() {
    let n = 100_000;
    let parens = format!("output {}1{};", "(".repeat(n), ")".repeat(n));
    let minus = format!("output {}1;", "-".repeat(n));
    let not = format!("output {}1;", "!".repeat(n));
    let sum = format!("output 1{};", " + 1".repeat(n));
    let product = format!("input x; output x{};", " * x".repeat(n));
    let calls = format!("fn f(a) = a;\noutput {}1{};", "f(".repeat(n), ")".repeat(n));
    let blocks = format!("output {}1{};", "{ ".repeat(n), " }".repeat(n));
    let loops = format!(
        "{}output 1;{}",
        "for i in 0..1 { ".repeat(n),
        " }".repeat(n)
    );
    for src in [parens, minus, not, sum, product, calls, blocks, loops] {
        let head: String = src.chars().take(24).collect();
        assert!(
            matches!(
                compiles_on_a_small_stack(src),
                Err(CompileError::NestingTooDeep { .. })
            ),
            "`{head}...` was not refused for its depth"
        );
    }
    /* Ordinary depth still compiles. */
    let fine = format!("output {}1{};", "(".repeat(64), ")".repeat(64));
    assert!(compiles_on_a_small_stack(fine).is_ok());
}

#[test]
fn inlining_deep_bodies_is_an_error_not_a_stack_overflow() {
    /*
     * Each body is shallow enough to parse, but inlining stacks them: two hundred
     * functions each negating a hundred times and calling the next.
     */
    let mut src = String::from("fn f0(a) = a;\n");
    for k in 1..200 {
        src.push_str(&format!("fn f{k}(a) = {}f{}(a);\n", "-".repeat(100), k - 1));
    }
    src.push_str("input x; output f199(x);");
    assert!(matches!(
        compiles_on_a_small_stack(src),
        Err(CompileError::RecursionTooDeep)
    ));
}
