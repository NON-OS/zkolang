/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A method call on a receiver that has no such method, or whose type is not known yet:
 * reported, and its arguments checked for their own errors.
 */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report that a receiver of type `rt` has no method `method`. */
    pub(super) fn no_method(
        &mut self,
        receiver: &'a Expr,
        method: &Ident,
        rt: TyId,
        args: &'a [Expr],
        at: Span,
    ) -> TExpr {
        let d = if let TyKind::Var(_) = self.kind(rt) {
            let what = format!(
                "the type of the value `{}` is called on is not known",
                method.name
            );
            let help = "give the literal a suffix, such as `2u32`, or bind it with a type first";
            let label = "an integer literal of no settled type";
            Diagnostic::error(Code::CANNOT_INFER, what, receiver.span, label).with_help(help)
        } else {
            let shown = self.show(rt);
            let what = format!("`{shown}` has no method `{}`", method.name);
            Diagnostic::error(Code::NO_FIELD, what, method.span, "no such method")
        };
        self.sema.diags.push(d);
        self.check_args_then_error(args, at)
    }
}
