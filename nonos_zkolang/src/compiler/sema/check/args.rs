/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Call arguments: a value for a value parameter, `&mut place` for a `&mut` one. */

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::tir::{Labels, TArg};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The argument `a` for the parameter `param`, if the call has one there. */
    pub(crate) fn arg(&mut self, a: &'a Expr, param: Option<&(TyId, Labels, bool)>) -> TArg {
        let Some(&(ty, _, by_ref)) = param else {
            return TArg::Value(self.infer(a, None));
        };
        match (&a.kind, by_ref) {
            (ExprKind::RefMut(place), true) => TArg::Place(self.place(place, Some(ty))),
            (ExprKind::RefMut(_), false) => {
                let d = Diagnostic::error(
                    Code::MISMATCHED_TYPES,
                    "this parameter takes a value, not `&mut`",
                    a.span,
                    "a place given for a value",
                );
                self.sema.diags.push(d);
                TArg::Value(self.error(a.span))
            }
            (_, true) => {
                let d = Diagnostic::error(
                    Code::MISMATCHED_TYPES,
                    "this parameter takes `&mut place`",
                    a.span,
                    "a value given for a `&mut` parameter",
                )
                .with_help("pass a mutable variable or a part of one: `&mut x`");
                self.sema.diags.push(d);
                TArg::Value(self.infer(a, None))
            }
            (_, false) => TArg::Value(self.expr(a, Some(ty))),
        }
    }
}
