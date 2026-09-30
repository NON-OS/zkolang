/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The nodes and errors of binary operator chains. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::{Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{BinOp, Expr, ExprKind};

impl<'a> Parser<'a> {
    /** The node for `first` followed by the operators and operands `rest`. */
    pub(super) fn binary_node(&mut self, first: Expr, rest: Vec<(BinOp, Expr)>) -> Expr {
        let end = rest.last().map_or(first.span, |(_, e)| e.span);
        let span = first.span.to(end);
        self.mk(ExprKind::Binary(Box::new(first), rest), span)
    }

    /** Report a comparison that follows another, as in `a < b < c`. */
    pub(super) fn chained_comparison(&mut self) -> Reported {
        let d = Diagnostic::error(
            Code::CHAINED_COMPARISON,
            "comparison operators cannot be chained",
            self.span(),
            "a second comparison here",
        )
        .with_help("write each comparison separately and join them with `&&`");
        self.diags.push(d);
        Reported
    }
}
