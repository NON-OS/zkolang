/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The `while` loop, which must state a bound on its iterations. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** `while cond limit N { .. }`. */
    pub(super) fn while_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        let cond = self.restricted(true, |p| p.expr())?;
        let limit_ok = self.at(TokenKind::Kw(Keyword::Limit));
        if !limit_ok {
            let d = Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                "a `while` loop needs a bound",
                self.span(),
                "expected `limit N` here",
            )
            .with_help("every loop unrolls at compile time: write `while cond limit 32 { ... }`");
            self.diags.push(d);
            return Err(Reported);
        }
        self.bump();
        let limit = self.const_arg()?;
        let body = self.block()?;
        let span = start.to(self.prev_span());
        Ok(self.mk(
            ExprKind::While {
                cond: Box::new(cond),
                limit,
                body: Box::new(body),
            },
            span,
        ))
    }
}
