/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The postfix chain of calls, indexing, fields and method calls, and argument lists. Each
 * link of the chain spends one nesting level while the chain is open, as an operator
 * chain does.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A primary followed by calls, indexing, fields and method calls. */
    pub(super) fn postfix(&mut self) -> PResult<Expr> {
        let base = self.depth;
        let r = self.postfix_links();
        self.depth = base;
        r
    }

    fn postfix_links(&mut self) -> PResult<Expr> {
        let mut e = self.primary()?;
        loop {
            match self.kind() {
                TokenKind::LParen => {
                    self.enter()?;
                    self.bump();
                    let args = self.args(TokenKind::RParen)?;
                    let span = e.span.to(self.prev_span());
                    e = self.mk(ExprKind::Call(Box::new(e), args), span);
                }
                TokenKind::LBracket => {
                    self.enter()?;
                    self.bump();
                    let index = self.restricted(false, |p| p.expr())?;
                    self.expect(TokenKind::RBracket)?;
                    let span = e.span.to(self.prev_span());
                    e = self.mk(ExprKind::Index(Box::new(e), Box::new(index)), span);
                }
                TokenKind::Dot => {
                    self.enter()?;
                    self.bump();
                    e = self.after_dot(e)?;
                }
                _ => return Ok(e),
            }
        }
    }

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
            p.expect(close)?;
            Ok(out)
        })
    }
}
