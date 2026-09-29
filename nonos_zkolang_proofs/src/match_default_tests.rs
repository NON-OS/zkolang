/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A match has exactly one default arm. A second `_` used to replace the first without a
 * word, and a missing one was reported at the token after the match.
 */

use nonos_zkolang::{compile_source, evaluate, CompileError};

#[test]
fn a_second_default_is_refused() {
    let src = "input x;\noutput match x { 0 => 10, _ => 20, _ => 30 };";
    let at = src.rfind('_').expect("second default");
    assert_eq!(
        compile_source(src).err(),
        Some(CompileError::UnexpectedToken { at })
    );
    let one = "input x;\noutput match x { 0 => 10, _ => 20 };";
    let ops = compile_source(one).expect("compile");
    assert_eq!(evaluate(&ops, &[5], &[]).expect("run"), vec![20]);
}

#[test]
fn a_missing_default_points_at_the_closing_brace() {
    let src = "input x;\noutput match x { 0 => 10, 1 => 20 } + 1;";
    let at = src.find('}').expect("closing brace");
    assert_eq!(
        compile_source(src).err(),
        Some(CompileError::UnexpectedToken { at })
    );
}
