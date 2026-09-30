/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `e as T` (section 7.8): allowed only where every value of the source type is a value of
 * `T`. Every other conversion is an error that names the explicit forms.
 */

use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
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
        let ok = match (self.kind(e.ty), &to_kind) {
            (TyKind::Error | TyKind::Never, _) | (_, TyKind::Error) => true,
            (from, to) if from == *to => true,
            (TyKind::Bool, TyKind::Int(_) | TyKind::Field) | (TyKind::Int(_), TyKind::Field) => {
                true
            }
            (TyKind::Int(f), TyKind::Int(t)) => f.fits_in(*t),
            (TyKind::Var(_), TyKind::Int(_) | TyKind::Field) => self.unify(e.ty, to),
            _ => false,
        };
        if !ok {
            let (from, shown) = (self.show(e.ty), self.show(to));
            let help = match to_kind {
                TyKind::Int(_) => format!("`{shown}::checked_from(e)` fails on a value `{shown}` does not hold; `{shown}::wrapping_from(e)` keeps the low bits"),
                TyKind::Bool => String::from("`bool::checked_from(e)` fails unless the value is 0 or 1"),
                _ => String::from("no conversion between these types is defined"),
            };
            let d = Diagnostic::error(
                Code::INVALID_CAST,
                format!("`{from}` does not convert to `{shown}` with `as`"),
                at,
                "not every value fits",
            )
            .with_help(help);
            self.sema.diags.push(d);
        }
        TExpr {
            kind: TExprKind::Cast(Box::new(e)),
            ty: to,
            span: at,
        }
    }
}
