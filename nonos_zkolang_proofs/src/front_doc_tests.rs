/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Doc comments document what follows them wherever they stand among its attributes, a
 * tuple field's doc is kept like a named field's, and taking them costs time linear in
 * the file.
 */

use std::time::{Duration, Instant};

use crate::front_lex_tests::parse;
use nonos_zkolang::compiler::syntax::ast::{Fields, ItemKind};

#[test]
fn docs_among_attributes_document_the_item() {
    let (ast, codes) = parse("/// A\n#[test]\n/// B\nfn g() {}\n#[test]\n/** C */\nfn h() {}");
    assert!(codes.is_empty(), "{codes:?}");
    assert_eq!(ast.items[0].doc.as_deref(), Some("A\nB"));
    assert_eq!(ast.items[1].doc.as_deref(), Some("C"));
    let (ast, codes) = parse("#![allow(x)]\n//! M\nfn f() {}");
    assert!(codes.is_empty(), "{codes:?}");
    assert_eq!(ast.inner_doc.as_deref(), Some("M"));
}

#[test]
fn a_tuple_field_keeps_its_doc() {
    let (ast, codes) = parse("struct S(\n    /// the count\n    u8,\n);");
    assert!(codes.is_empty(), "{codes:?}");
    let ItemKind::Struct(s) = &ast.items[0].kind else {
        panic!("not a struct");
    };
    let Fields::Tuple(fields) = &s.fields else {
        panic!("not a tuple struct");
    };
    assert_eq!(fields[0].doc.as_deref(), Some("the count"));
}

#[test]
fn a_misplaced_inner_attribute_is_one_error() {
    assert_eq!(parse("fn f() {}\n#![allow(x)]\nfn g() {}").1, vec!["E0100"]);
}

#[test]
fn taking_docs_is_linear() {
    let src: String = (0..100_000)
        .map(|i| format!("// c\nconst C{i}: u8 = 1;\n"))
        .collect();
    let t = Instant::now();
    assert!(parse(&src).1.is_empty());
    assert!(t.elapsed() < Duration::from_secs(20), "{:?}", t.elapsed());
}
