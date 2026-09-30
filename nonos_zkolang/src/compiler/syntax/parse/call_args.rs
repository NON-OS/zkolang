/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Argument lists: a call's, a method call's, and a struct literal's. */

use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::Expr;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A comma-separated argument list up to `close`, which it consumes. */
    pub(super) fn args(&mut self, close: TokenKind) -> PResult<Vec<Expr>> {
        self.restricted(false, |p| {
            let mut out = Vec::new();
            while !p.at(close) {
                if p.at(TokenKind::Eof) {
                    return Err(p.unexpected(close.describe()));
                }
                out.push(p.expr()?);
                if !p.eat(TokenKind::Comma) {
                    break;
                }
            }
            p.expect_list_end(close)?;
            Ok(out)
        })
    }
}
