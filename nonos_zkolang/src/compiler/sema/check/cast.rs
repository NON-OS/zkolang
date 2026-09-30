/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `e as T` (section 7.8): allowed only where every value of the source type is a value of
 * `T`. An operand whose literal type is still open is checked once that type is settled,
 * so the cast does not give it a type the rest of the body contradicts.
 */

use alloc::boxed::Box;

use super::cx::FnCx;
use super::deferred::Deferred;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Type};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `a as ty`. */
    pub(crate) fn cast(&mut self, a: &'a Expr, ty: &'a Type, at: Span) -> TExpr {
        let (to, _) = self.sema.lower_ty(self.module, ty);
        let to_kind = self.kind(to);
        let numeric = matches!(to_kind, TyKind::Int(_) | TyKind::Field);
        let e = self.infer(a, numeric.then_some(to));
        let from = match numeric {
            true => self.numeric(e.ty),
            false => e.ty,
        };
        let from = self.kind(from);
        if numeric && matches!(from, TyKind::Var(_)) {
            self.deferred.push(Deferred::Cast {
                ty: e.ty,
                to,
                span: at,
            });
        } else if !castable(&from, &to_kind) {
            self.bad_cast(e.ty, to, at);
        }
        TExpr {
            kind: TExprKind::Cast(Box::new(e)),
            ty: to,
            span: at,
        }
    }
}

/** Whether every value of `from` is a value of `to`. */
pub(super) fn castable(from: &TyKind, to: &TyKind) -> bool {
    match (from, to) {
        (TyKind::Error | TyKind::Never, _) | (_, TyKind::Error) => true,
        (f, t) if f == t => true,
        (TyKind::Bool, TyKind::Int(_) | TyKind::Field) | (TyKind::Int(_), TyKind::Field) => true,
        (TyKind::Int(f), TyKind::Int(t)) => f.fits_in(*t),
        _ => false,
    }
}
