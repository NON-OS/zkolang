/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The nodes and errors of binary operator chains. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{BinOp, Expr, ExprKind};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** The node for `first` followed by the operators and operands `rest`. */
    pub(super) fn binary_node(&mut self, first: Expr, rest: Vec<(BinOp, Expr)>) -> Expr {
        let end = rest.last().map_or(first.span, |(_, e)| e.span);
        let span = first.span.to(end);
        self.mk(ExprKind::Binary(Box::new(first), rest), span)
    }

    /**
     * Report the comparison `op` that follows another, as in `a < b < c`. The help names
     * the turbofish when the text reads as a call with generic arguments, `f<T>(x)`.
     */
    pub(super) fn chained_comparison(
        &mut self,
        op: BinOp,
        lhs: &Expr,
        rest: Option<&[(BinOp, Expr)]>,
    ) {
        let generic = op == BinOp::Gt
            && matches!(lhs.kind, ExprKind::Path(_))
            && self.peek(1) == TokenKind::LParen
            && matches!(rest, Some([(BinOp::Lt, _)]));
        let help = if generic {
            "to call a function with generic arguments, write `f::<T>(x)`"
        } else {
            "write each comparison separately and join them with `&&`"
        };
        let d = Diagnostic::error(
            Code::CHAINED_COMPARISON,
            "comparison operators cannot be chained",
            self.span(),
            "a second comparison here",
        )
        .with_help(help);
        self.diags.push(d);
    }
}
