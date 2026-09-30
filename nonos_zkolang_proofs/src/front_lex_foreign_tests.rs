/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Literal forms of other languages are each one error: single-quoted text, whatever it
 * holds, and Rust's raw and byte strings. Text left unterminated hides the separator it
 * swallowed, and nothing else.
 */

use crate::front_lex_tests::parse;

#[test]
fn single_quoted_text_is_one_error() {
    for src in [
        "fn f() { assert x, 'value too large'; }",
        "fn f() { let e = ''; }",
        "fn f() { let n = '1 2'; }",
    ] {
        assert_eq!(parse(src).1, vec!["E0001"], "{src}");
    }
    assert_eq!(parse("fn g(x: &'a u8, y: &'b u8) {}").1, vec!["E0001"; 2]);
}

#[test]
fn raw_and_byte_strings_are_one_error_each() {
    let src = "fn f() {\n    let s = r#\"abc\"#;\n    let t = r\"x\";\n    let u = b\"y\\\"z\";\n    let w = r#\"multi\nline\"#;\n}\n";
    assert_eq!(parse(src).1, vec!["E0001"; 4]);
}

#[test]
fn an_unterminated_string_hides_only_the_separator_it_swallowed() {
    for open in ["\"abc;", "r#\"abc;"] {
        let src = format!("fn f() {{\n    let s = {open}\n    let q = 1 +;\n}}\n");
        let first = if open.starts_with('r') {
            "E0001"
        } else {
            "E0003"
        };
        assert_eq!(parse(&src).1, vec![first, "E0100"], "{src}");
    }
}
