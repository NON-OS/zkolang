/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parser's entry points. */

use super::parser::Parser;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::ast::{Expr, SourceAst};
use crate::compiler::syntax::lex::Lexed;
use crate::compiler::syntax::placement::check_items;
use crate::compiler::syntax::token::TokenKind;

/**
 * Parse one file. The result always exists; problems are in `diags`. `next_id` numbers
 * the nodes and is shared across the files of one compilation so ids stay unique.
 */
pub fn parse_file(
    file: FileId,
    text: &str,
    lexed: &Lexed,
    diags: &mut Diagnostics,
    next_id: &mut u32,
) -> SourceAst {
    let mut p = Parser::new(file, text, lexed, diags, next_id);
    let (inner_doc, inner_attrs) = p.inner_doc_and_attrs(0).unwrap_or_else(|_| {
        p.recover_item(0);
        Default::default()
    });
    let items = p.items(false);
    p.warn_unused_docs();
    check_items(&items, p.diags);
    let end = u32::try_from(text.len()).unwrap_or(u32::MAX);
    SourceAst {
        file,
        inner_attrs,
        inner_doc,
        items,
        span: Span::new(file, 0, end),
    }
}

/**
 * Parse a single expression filling the whole text, for tests and tools. `None` when it
 * does not parse or leaves tokens over; problems are in `diags`.
 */
pub fn parse_expr(
    file: FileId,
    text: &str,
    lexed: &Lexed,
    diags: &mut Diagnostics,
    next_id: &mut u32,
) -> Option<Expr> {
    let mut p = Parser::new(file, text, lexed, diags, next_id);
    let e = p.expr().ok()?;
    if p.kind() != TokenKind::Eof {
        p.unexpected("the end of the expression");
        return None;
    }
    Some(e)
}
