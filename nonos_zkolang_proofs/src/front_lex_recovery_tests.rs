/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lexical mistakes that must not hide or invent others: a tuple index with underscores is
 * valid, an unterminated string takes at most the next line, a space look-alike leaves
 * the error before it reported, and text left open hides only what it swallowed.
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
    for space in [
        '\u{a0}', '\u{2003}', '\u{202f}', '\u{205f}', '\u{1680}', '\u{b}',
    ] {
        let src = format!("fn f() -> u8 {{\n    let x = 1\n{space}   let y = 2;\n    x + y\n}}\n");
        assert_eq!(parse(&src).1, vec!["E0001", "E0100"], "{space:?}");
    }
}

#[test]
fn text_left_open_hides_only_what_it_swallowed() {
    for src in [
        "struct S {\n    a: u8, /* note\n}\nfn g() {}\n",
        "fn f(a: u8, /* x\n",
        "fn f() {\n    if x { /* c\n",
    ] {
        assert_eq!(parse(src).1, vec!["E0002"], "{src:?}");
    }
    let later = "fn f() {\n    let x = 1;\nfn g() {}\n/* unterminated\n";
    assert_eq!(parse(later).1, vec!["E0002", "E0101"]);
    let no_brace = "fn f() {\n    assert x, \"a\n    let y = 1;\nfn g() {}\n";
    assert_eq!(parse(no_brace).1, vec!["E0003", "E0101"]);
}
