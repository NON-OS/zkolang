/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The `while` loop, which must state a bound on its iterations. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{ConstArg, Expr, ExprKind};
use crate::compiler::syntax::keyword::Keyword;

impl<'a> Parser<'a> {
    /**
     * `while cond limit N { .. }`. A missing bound is reported and the body still read,
     * with an error expression standing for the bound.
     */
    pub(super) fn while_expr(&mut self) -> PResult<Expr> {
        let start = self.span();
        self.bump();
        if self.at_kw(Keyword::Let) {
            return Err(self.while_let());
        }
        let cond = self.restricted(true, |p| p.expr())?;
        let limit = if !self.eat_kw(Keyword::Limit) {
            self.no_bound("a `while` loop needs a bound", "expected `limit N` here")
        } else if self.limit_without_bound() {
            self.no_bound(
                "`limit` needs a bound",
                "the loop's body, with no bound before it",
            )
        } else {
            self.const_arg()?
        };
        let body = self.body_block()?;
        let span = start.to(self.prev_span());
        let (cond, body) = (Box::new(cond), Box::new(body));
        Ok(self.mk(ExprKind::While { cond, limit, body }, span))
    }

    /** Report a loop with no bound, and the error expression that stands for it. */
    fn no_bound(&mut self, message: &str, label: &str) -> ConstArg {
        let at = self.span();
        let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, message, at, label)
            .with_help("every loop unrolls at compile time: write `while cond limit 32 { ... }`");
        self.diags.push(d);
        ConstArg::Expr(Box::new(self.mk(ExprKind::Error, at)))
    }
}
