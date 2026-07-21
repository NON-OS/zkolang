/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Diagnostics: a syntax error is reported with its line, its column, and a caret under
//! the offending place in the source, not just a kind. This is the difference between a
//! compiler and a toy.

use nonos_zkolang::{compile_source, render_error};

fn diag(src: &str) -> String {
    let e = compile_source(src).expect_err("expected a compile error");
    render_error(src, &e)
}

#[test]
fn an_unexpected_token_points_at_its_line_and_column() {
    let d = diag("input x;\nlet y = x * ;\noutput y;");
    assert!(d.contains("unexpected token"), "{d}");
    assert!(d.contains("2:13"), "wrong location:\n{d}");
    assert!(d.contains('^'), "no caret:\n{d}");
}

#[test]
fn a_number_too_large_points_at_the_number() {
    let d = diag("output 999999999999999999999999;");
    assert!(d.contains("number too large"), "{d}");
    assert!(d.contains("1:8"), "wrong location:\n{d}");
}

#[test]
fn an_unexpected_character_points_at_it() {
    let d = diag("output x @ y;");
    assert!(d.contains("unexpected character"), "{d}");
    assert!(d.contains("1:10"), "wrong location:\n{d}");
}

#[test]
fn a_missing_semicolon_is_flagged_where_the_next_statement_begins() {
    let d = diag("input x\noutput x;");
    assert!(d.contains("unexpected token"), "{d}");
    assert!(d.contains("2:1"), "wrong location:\n{d}");
}

#[test]
fn a_semantic_error_names_the_offending_symbol() {
    // An unknown variable quotes the name and points a caret at its use.
    let d = diag("output foo + 1;");
    assert!(d.contains("unknown variable"), "{d}");
    assert!(d.contains("`foo`"), "name not quoted:\n{d}");
    assert!(
        d.contains("1:8"),
        "unknown variable not located at its use:\n{d}"
    );
    assert!(d.contains('^'), "no caret:\n{d}");
    // An arity mismatch names the function and both counts. Its name is defined, so it is
    // reported without a location rather than pointing a caret at the definition.
    let a = diag("fn f(x) = x; output f(1, 2);");
    assert!(a.contains("`f`"), "function not named:\n{a}");
    assert!(a.contains('1') && a.contains('2'), "counts missing:\n{a}");
    assert!(
        !a.contains("-->"),
        "arity mismatch should not point at the definition:\n{a}"
    );
}

#[test]
fn an_unknown_name_points_at_the_use_and_skips_a_comment() {
    // The caret lands on the first real occurrence of the name, its use. The name in the
    // comment is not a token, so it is skipped and the location is the third line.
    let d = diag("// foo is only mentioned here\ninput x;\nlet y = foo + x;\noutput y;");
    assert!(d.contains("unknown variable"), "{d}");
    assert!(d.contains("3:9"), "did not point at the use:\n{d}");
    assert!(d.contains('^'), "no caret:\n{d}");
}
