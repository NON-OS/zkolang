/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The `if` expression and its `else if` and `else` branches. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::keyword::Keyword;

impl<'a> Parser<'a> {
    /** `if cond { .. } else if .. else { .. }`. */
    pub(super) fn if_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        let cond = self.restricted(true, |p| p.expr())?;
        let then_block = self.block()?;
        let else_branch = if self.eat_kw(Keyword::Else) {
            if self.at_kw(Keyword::If) {
                Some(Box::new(self.nested(|p| p.if_expr())?))
            } else {
                let b = self.block()?;
                let span = b.span;
                Some(Box::new(self.mk(ExprKind::Block(Box::new(b)), span)))
            }
        } else {
            None
        };
        let span = start.to(self.prev_span());
        Ok(self.mk(
            ExprKind::If {
                cond: Box::new(cond),
                then_block: Box::new(then_block),
                else_branch,
            },
            span,
        ))
    }
}
