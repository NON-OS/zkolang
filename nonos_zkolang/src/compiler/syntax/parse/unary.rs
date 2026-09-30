/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Casts and prefix operators. Each link of a cast chain spends one nesting level while
 * the chain is open, as an operator chain does.
 */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind, UnOp};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `e as T as U ...` */
    pub(super) fn cast(&mut self) -> PResult<Expr> {
        let base = self.depth;
        let r = self.cast_links();
        self.depth = base;
        r
    }

    fn cast_links(&mut self) -> PResult<Expr> {
        let mut e = self.unary()?;
        while self.eat_kw(Keyword::As) {
            self.enter()?;
            let ty = self.ty()?;
            let span = e.span.to(ty.span);
            e = self.mk(ExprKind::Cast(Box::new(e), ty), span);
        }
        Ok(e)
    }

    /** Prefix `-`, `!` and `&mut`. */
    fn unary(&mut self) -> PResult<Expr> {
        let start = self.span();
        let op = match self.kind() {
            TokenKind::Minus => Some(UnOp::Neg),
            TokenKind::Bang => Some(UnOp::Not),
            _ => None,
        };
        if let Some(op) = op {
            self.bump();
            let inner = self.nested(|p| p.unary())?;
            let span = start.to(inner.span);
            return Ok(self.mk(ExprKind::Unary(op, Box::new(inner)), span));
        }
        if self.at(TokenKind::Amp) {
            self.bump();
            if !self.eat_kw(Keyword::Mut) {
                return Err(
                    self.unexpected("`mut`: arguments are passed by value or as `&mut place`")
                );
            }
            let inner = self.nested(|p| p.unary())?;
            let span = start.to(inner.span);
            return Ok(self.mk(ExprKind::RefMut(Box::new(inner)), span));
        }
        self.postfix()
    }
}
