/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Where a syntax error points: at the token found, except a missing separator or closer,
 * which is reported after the last token of its line, and anything missing at the end of
 * the file, which is reported after the last token.
 */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

/** The message, line and column of the first diagnostic for `src`. */
fn first(src: &str) -> (String, usize, usize) {
    let mut map = SourceMap::new();
    let id = map.add(String::from("f.zkl"), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    parse_file(id, src, &lexed, &mut diags, &mut 0);
    let d = diags.items().first().expect("a diagnostic").clone();
    let (_, line, col) = map.locate(d.span()).expect("in the map");
    (d.message, line, col)
}

#[test]
fn a_misplaced_token_is_reported_where_it_is() {
    let (_, line, col) = first("fn f() {}\n\nlet x = 1;\n");
    assert_eq!((line, col), (3, 1));
    let (_, line, col) = first("fn f() {\n    let x = 1;\n    =>\n}\n");
    assert_eq!((line, col), (3, 5));
}

#[test]
fn a_missing_separator_is_reported_after_its_line() {
    let (message, line, col) = first("struct S {\n    a: u8\n    b: u8,\n}\n");
    assert_eq!(message, "expected `,` or `}`, found `b`");
    assert_eq!((line, col), (2, 10));
    let (message, line, col) = first("fn f() -> u32 { g(a b) }");
    assert_eq!(message, "expected `,` or `)`, found `b`");
    assert_eq!((line, col), (1, 21));
}

#[test]
fn what_is_missing_at_the_end_is_reported_after_the_last_token() {
    let (_, line, col) = first("const C: u8 = 1 // c");
    assert_eq!((line, col), (1, 16));
}
