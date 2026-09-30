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
        let e = self.primary()?;
        self.postfix_on(e)
    }

    /**
     * A block-like expression `e` that stands where it would end a statement or an arm,
     * continued if a `.` follows it: `match x { .. }.len()`.
     */
    pub(super) fn after_block_like(&mut self, e: Expr) -> PResult<Expr> {
        if !self.at(TokenKind::Dot) {
            return Ok(e);
        }
        let base = self.depth;
        let r = self.postfix_on(e);
        self.depth = base;
        r
    }

    /** The calls, indexing, fields and method calls after `e`. */
    fn postfix_on(&mut self, mut e: Expr) -> PResult<Expr> {
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
}
