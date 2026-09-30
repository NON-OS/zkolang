/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lexical mistakes that must not hide or invent others: a tuple index with underscores is
 * valid, an unterminated string takes at most the next line, and a space look-alike
 * leaves the error before it reported.
 */

use crate::front_lex_tests::parse;

#[test]
fn a_tuple_index_may_hold_underscores() {
    assert_eq!(parse("fn f() { let x = t.1_0; }").1, Vec::<&str>::new());
}

#[test]
fn an_unterminated_string_takes_a_quote_on_the_next_line_only() {
    let src = "fn f(x: bool) {\n    assert x, \"abc;\n    let y = 1;\n    /* \"q\" */\n}\n";
    assert_eq!(parse(src).1, vec!["E0003"]);
}

#[test]
fn a_space_look_alike_hides_no_other_error() {
    let src = "fn f() -> u8 {\n    let x = 1\n\u{a0}   let y = 2;\n    x + y\n}\n";
    assert_eq!(parse(src).1, vec!["E0001", "E0100"]);
}
