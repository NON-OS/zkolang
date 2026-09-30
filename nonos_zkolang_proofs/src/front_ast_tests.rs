/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The shape of the tree: spans end where their text ends even when a `>>` is split, a
 * constant argument keeps its suffix, and operator and `else if` chains are flat.
 */

use crate::front_lex_tests::parse;
use nonos_zkolang::compiler::syntax::ast::{ConstArg, ExprKind, ItemKind, TypeKind};
use nonos_zkolang::compiler::syntax::IntTy;

#[test]
fn a_split_closer_ends_each_type_at_its_own_angle() {
    let src = "const X: C<D<u8>>= 1;";
    let (ast, codes) = parse(src);
    assert!(codes.is_empty(), "{codes:?}");
    let ItemKind::Const(c) = &ast.items[0].kind else {
        panic!("not a const");
    };
    let text = |s: nonos_zkolang::compiler::source::Span| &src[s.lo as usize..s.hi as usize];
    assert_eq!(text(c.ty.span), "C<D<u8>>");
    let TypeKind::Path(p) = &c.ty.kind else {
        panic!("not a path");
    };
    let inner = &p.segments[0].generics.as_ref().expect("arguments")[0];
    let nonos_zkolang::compiler::syntax::ast::GenericArg::Type(d) = inner else {
        panic!("not a type argument");
    };
    assert_eq!(text(d.span), "D<u8>");
}

#[test]
fn a_constant_argument_keeps_its_suffix() {
    let (ast, _) = parse("type T = [u8; 3u8];");
    let ItemKind::TypeAlias(a) = &ast.items[0].kind else {
        panic!("not an alias");
    };
    let TypeKind::Array(_, ConstArg::Lit { value, suffix, .. }) = &a.ty.kind else {
        panic!("not an array of a literal length");
    };
    assert_eq!((*value, *suffix), (3, Some(IntTy::U8)));
}

#[test]
fn chains_are_flat() {
    let (ast, codes) = parse("fn f() { a + b - c + d; if a { } else if b { } else { } }");
    assert!(codes.is_empty(), "{codes:?}");
    let ItemKind::Fn(f) = &ast.items[0].kind else {
        panic!("not a fn");
    };
    let nonos_zkolang::compiler::syntax::ast::StmtKind::Expr { expr, .. } = &f.body.stmts[0].kind
    else {
        panic!("not an expression statement");
    };
    assert!(matches!(&expr.kind, ExprKind::Binary(_, rest) if rest.len() == 3));
    let tail = f.body.tail.as_deref().expect("the if is the tail");
    assert!(matches!(&tail.kind, ExprKind::If(b, Some(_)) if b.len() == 2));
}
