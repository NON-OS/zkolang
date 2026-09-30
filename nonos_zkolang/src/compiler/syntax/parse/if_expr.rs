/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The `if` expression and its `else if` and `else` branches. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{PResult, Parser};
use crate::compiler::syntax::ast::{Expr, ExprKind, IfBranch};
use crate::compiler::syntax::keyword::Keyword;

impl<'a> Parser<'a> {
    /**
     * `if cond { .. } else if .. else { .. }`. An `else if` chain is one node with a branch
     * per condition, so a long chain costs no nesting.
     */
    pub(super) fn if_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        let mut branches = Vec::new();
        let mut else_block = None;
        loop {
            self.bump();
            if self.at_kw(Keyword::Let) {
                return Err(self.if_let());
            }
            let cond = self.restricted(true, |p| p.expr())?;
            let block = self.body_block()?;
            branches.push(IfBranch { cond, block });
            if !self.eat_kw(Keyword::Else) {
                break;
            }
            if !self.at_kw(Keyword::If) {
                else_block = Some(Box::new(self.block()?));
                break;
            }
        }
        let span = start.to(self.prev_span());
        Ok(self.mk(ExprKind::If(branches, else_block), span))
    }
}
