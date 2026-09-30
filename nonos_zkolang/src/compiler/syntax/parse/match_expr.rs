/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The `match` expression and its arms. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use super::stmt_expr::block_like;
use crate::compiler::syntax::ast::{Arm, Expr, ExprKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `match scrutinee { pat if guard => body, ... }`. */
    pub(super) fn match_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        let scrutinee = self.restricted(true, |p| p.expr())?;
        self.expect(TokenKind::LBrace)?;
        let arms = self.restricted(false, |p| p.arms())?;
        self.expect(TokenKind::RBrace)?;
        let span = start.to(self.prev_span());
        Ok(self.mk(
            ExprKind::Match {
                scrutinee: Box::new(scrutinee),
                arms,
            },
            span,
        ))
    }

    fn arms(&mut self) -> PResult<Vec<Arm>> {
        let mut arms = Vec::new();
        while !self.at(TokenKind::RBrace) {
            if self.at(TokenKind::Eof) {
                return Err(self.unexpected("`}`"));
            }
            let start = self.span();
            let pat = self.pattern()?;
            let guard = if self.eat_kw(Keyword::If) {
                Some(self.expr()?)
            } else {
                None
            };
            self.expect(TokenKind::FatArrow)?;
            let body = if self.at(TokenKind::LBrace) {
                let b = self.block()?;
                let span = b.span;
                let e = self.mk(ExprKind::Block(Box::new(b)), span);
                self.after_block_like(e)?
            } else {
                self.expr()?
            };
            /* After a block-like body, as after a block statement, the comma may be left out. */
            let braced = block_like(&body);
            arms.push(Arm {
                pat,
                guard,
                span: start.to(body.span),
                body,
            });
            if !self.eat(TokenKind::Comma) && !braced && !self.at(TokenKind::RBrace) {
                return Err(self.unexpected("`,` between match arms"));
            }
        }
        Ok(arms)
    }
}
