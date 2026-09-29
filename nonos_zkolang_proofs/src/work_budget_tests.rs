/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Compilation does bounded work on any input. Straight-line inlining that doubles per
 * level, nested loops whose bodies emit nothing, and many statements each just under the
 * desugaring budget used to run for hours or exhaust memory, because the instruction cap
 * was only checked inside loops and the node budget was per expression.
 */

use crate::small_stack::compiles_on_a_small_stack;
use nonos_zkolang::CompileError;

#[test]
fn doubling_inlines_stop_at_the_instruction_cap() {
    let mut src = String::from("fn f0(x) = x + 1;\n");
    for k in 1..31 {
        src.push_str(&format!("fn f{k}(x) = f{j}(x) + f{j}(x);\n", j = k - 1));
    }
    src.push_str("input x;\noutput f30(x);");
    assert_eq!(
        compiles_on_a_small_stack(src).err(),
        Some(CompileError::ProgramTooLong)
    );
    /* Calls that emit nothing still cost work. */
    let mut ident = String::from("fn g0(x) = x;\n");
    for k in 1..41 {
        ident.push_str(&format!("fn g{k}(x) = g{j}(g{j}(x));\n", j = k - 1));
    }
    ident.push_str("input x;\noutput g40(x);");
    assert_eq!(
        compiles_on_a_small_stack(ident).err(),
        Some(CompileError::ProgramTooLong)
    );
}

#[test]
fn loops_that_emit_nothing_stop_at_the_iteration_cap() {
    let src = "for i in 0..65536 { for j in 0..65536 { for k in 0..65536 { } } }\noutput 1;";
    assert_eq!(
        compiles_on_a_small_stack(src.to_string()).err(),
        Some(CompileError::ProgramTooLong)
    );
}

#[test]
fn the_desugaring_budget_covers_the_whole_program() {
    let line = format!("output a{};\n", " || a".repeat(14));
    let src = format!("input a;\n{}", line.repeat(400));
    assert!(matches!(
        compiles_on_a_small_stack(src),
        Err(CompileError::ExpressionTooLarge { .. })
    ));
}

#[test]
fn a_long_flat_chain_still_compiles() {
    /* A chain of two hundred terms is well within the nesting budget of 256. */
    let sum = format!("input x;\noutput x{};", " + x".repeat(200));
    assert!(compiles_on_a_small_stack(sum).is_ok());
    let too_long = format!("input x;\noutput x{};", " + x".repeat(300));
    assert!(matches!(
        compiles_on_a_small_stack(too_long),
        Err(CompileError::NestingTooDeep { .. })
    ));
}
