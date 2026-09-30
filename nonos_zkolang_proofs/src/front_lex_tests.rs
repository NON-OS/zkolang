/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2026 lexer reads every line ending an editor displays, skips a byte-order
 * mark, reports a character that begins no token, and takes block doc comments as it
 * takes line doc comments.
 */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::ast::SourceAst;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

/** The tree and the codes of every diagnostic for one source. */
pub(crate) fn parse(src: &str) -> (SourceAst, Vec<&'static str>) {
    let mut map = SourceMap::new();
    let id = map.add(String::from("t.zkl"), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    let ast = parse_file(id, src, &lexed, &mut diags, &mut 0);
    (ast, diags.items().iter().map(|d| d.code.0).collect())
}

#[test]
fn every_line_ending_ends_a_line_comment() {
    for src in ["// c\nfn g() {}", "// c\r\nfn g() {}", "// c\rfn g() {}"] {
        let (ast, codes) = parse(src);
        assert_eq!((ast.items.len(), codes), (1, vec![]), "{src:?}");
    }
    let (ast, codes) = parse("\u{feff}fn f() {}\r\nfn g() {}\r");
    assert_eq!((ast.items.len(), codes), (2, vec![]));
    assert_eq!(parse("fn f() { let c = @; }").1, vec!["E0001"]);
}

#[test]
fn block_doc_comments_document_items_and_modules() {
    let src = "/*! The module. */\n/**\n * Adds one.\n *\n * Twice.\n */\nfn f() {}";
    let (ast, codes) = parse(src);
    assert!(codes.is_empty(), "{codes:?}");
    assert_eq!(ast.inner_doc.as_deref(), Some("The module."));
    assert_eq!(ast.items[0].doc.as_deref(), Some("Adds one.\n\nTwice."));
    let (ast, codes) = parse("/*** rule */\n/**/\nfn f() {}");
    assert_eq!((ast.items[0].doc.as_deref(), codes), (None, vec![]));
    let (_, codes) = parse("fn f() { /** nothing */ }");
    assert_eq!(codes, vec!["W0007"]);
}

#[test]
fn a_carriage_return_ends_a_string_line_and_stray_runs_are_one_error() {
    assert_eq!(parse("fn f() { assert true, \"a\rb\"; }").1, vec!["E0003"]);
    assert_eq!(
        parse("fn f() { assert true, \"a\rb; }\rfn g() {}").1,
        vec!["E0003"]
    );
    let run = format!("fn f() {{ let x = {}y; }}", "$".repeat(100_000));
    assert_eq!(parse(&run).1, vec!["E0001"]);
}

#[test]
fn a_tuple_index_may_hold_underscores() {
    assert_eq!(parse("fn f() { let x = t.1_0; }").1, Vec::<&str>::new());
}
