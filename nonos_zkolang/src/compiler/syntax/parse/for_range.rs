/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A `for` range whose upper bound is missing. */

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Expr, ExprKind};

impl<'a> Parser<'a> {
    /** Report a range with no upper bound, before the body, and stand an error for it. */
    pub(super) fn range_without_end(&mut self) -> Expr {
        let at = self.span();
        let d = Diagnostic::error(
            Code::UNEXPECTED_TOKEN,
            "a range needs an upper bound",
            at,
            "the loop's body",
        )
        .with_help("every loop unrolls at compile time: write `for i in 0..N { ... }`");
        self.diags.push(d);
        self.mk(ExprKind::Error, at)
    }
}
