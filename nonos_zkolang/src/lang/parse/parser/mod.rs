/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Recursive-descent parser: a token stream to the abstract syntax tree. Precedence
//! is encoded by the call chain, and one grammar rule lives per file: the cursor
//! primitives, the top-level items, the statements, and the expression levels.

mod and;
mod array_expr;
mod atom;
mod block;
mod const_def;
mod cursor;
mod equality;
mod expr;
mod fn_def;
mod for_loop;
mod ident;
mod ident_expr;
mod if_expr;
mod input_secret;
mod inv_expr;
mod match_expr;
mod number;
mod or;
mod primary;
mod product;
mod program;
mod sel_expr;
mod stmt;
mod sum;
mod unary;

use super::super::lex::Tok;
use super::super::CompileError;
use super::ast::Ast;

pub(crate) struct Parser<'a> {
    pub(crate) toks: &'a [Tok],
    pub(crate) spans: &'a [usize],
    pub(crate) eof: usize,
    pub(crate) pos: usize,
}

/// Parse a token stream into an AST. `spans` holds each token's byte offset and `eof`
/// is the length of the source, so a diagnostic can point past the last token.
pub fn parse(toks: &[Tok], spans: &[usize], eof: usize) -> Result<Ast, CompileError> {
    let mut p = Parser {
        toks,
        spans,
        eof,
        pos: 0,
    };
    p.program()
}
