/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The desugarings that copy their operands, `||` and `match`, stay within a node budget. */

use crate::small_stack::compiles_on_a_small_stack;
use nonos_zkolang::CompileError;

#[test]
fn a_desugaring_that_duplicates_is_bounded() {
    /*
     * `a || b` is `a + b - a * b`, so each `||` copies both sides and a chain doubles per
     * operator. Forty operators is a trillion nodes; it must be refused, not built.
     */
    let or = format!("input a; output a{};", " || a".repeat(40));
    assert!(matches!(
        compiles_on_a_small_stack(or),
        Err(CompileError::ExpressionTooLarge { .. })
    ));
    /* A match copies its scrutinee into every arm, so nested matches multiply. */
    let mut m = String::from("a");
    for _ in 0..12 {
        m = format!("match {m} {{ 0 => 1, 1 => 2, 2 => 3, 3 => 4, _ => 5 }}");
    }
    let nested = format!("input a; output {m};");
    assert!(matches!(
        compiles_on_a_small_stack(nested),
        Err(CompileError::ExpressionTooLarge { .. })
    ));
    /* Short chains compile as before. */
    let short = "input a; input b; input c; output a || b || c;".to_string();
    assert!(compiles_on_a_small_stack(short).is_ok());
}
