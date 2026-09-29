/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parser's entry point. */

use super::state::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Ast;
use crate::lang::CompileError;

/**
 * Parse a token stream into an AST. `spans` holds each token's byte offset and `eof`
 * is the length of the source, so a diagnostic can point past the last token.
 */
pub fn parse(toks: &[Tok], spans: &[usize], eof: usize) -> Result<Ast, CompileError> {
    let mut p = Parser {
        toks,
        spans,
        eof,
        pos: 0,
        depth: 0,
        copied: 0,
    };
    p.program()
}
