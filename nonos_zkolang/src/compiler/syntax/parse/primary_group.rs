/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bracketed primary expressions: parentheses, tuples and arrays. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `()`, `(e)`, `(e,)` or `(a, b, ...)`. */
    pub(super) fn paren_or_tuple(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        self.restricted(false, |p| {
            if p.eat(TokenKind::RParen) {
                let span = start.to(p.prev_span());
                return Ok(p.mk(ExprKind::Unit, span));
            }
            let first = p.expr()?;
            if p.eat(TokenKind::RParen) {
                let span = start.to(p.prev_span());
                return Ok(p.mk(ExprKind::Paren(Box::new(first)), span));
            }
            let mut elems = alloc::vec![first];
            while p.eat(TokenKind::Comma) {
                if p.at(TokenKind::RParen) {
                    break;
                }
                elems.push(p.expr()?);
            }
            p.expect(TokenKind::RParen)?;
            let span = start.to(p.prev_span());
            Ok(p.mk(ExprKind::Tuple(elems), span))
        })
    }

    /** `[a, b, ...]` or `[value; count]`. */
    pub(super) fn array(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        self.restricted(false, |p| {
            if p.eat(TokenKind::RBracket) {
                let span = start.to(p.prev_span());
                return Ok(p.mk(ExprKind::Array(Vec::new()), span));
            }
            let first = p.expr()?;
            if p.eat(TokenKind::Semi) {
                let count = p.const_arg()?;
                p.expect(TokenKind::RBracket)?;
                let span = start.to(p.prev_span());
                return Ok(p.mk(ExprKind::Repeat(Box::new(first), count), span));
            }
            let mut elems = alloc::vec![first];
            while p.eat(TokenKind::Comma) {
                if p.at(TokenKind::RBracket) {
                    break;
                }
                elems.push(p.expr()?);
            }
            p.expect(TokenKind::RBracket)?;
            let span = start.to(p.prev_span());
            Ok(p.mk(ExprKind::Array(elems), span))
        })
    }
}
