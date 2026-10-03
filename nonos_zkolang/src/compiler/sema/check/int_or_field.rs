/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The operand of a conversion: an integer or a `field`. */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `a`, which must be an integer or a `field`. */
    pub(super) fn int_or_field(&mut self, a: &'a Expr) -> TExpr {
        let e = self.infer(a, None);
        let ty = self.numeric(e.ty);
        match self.kind(ty) {
            TyKind::Int(_) | TyKind::Field | TyKind::Var(_) | TyKind::Error | TyKind::Never => {}
            _ => {
                let shown = self.show(e.ty);
                let d = Diagnostic::error(
                    Code::MISMATCHED_TYPES,
                    "mismatched types",
                    e.span,
                    format!("expected an integer or `field`, found `{shown}`"),
                );
                self.sema.diags.push(d);
            }
        }
        e
    }

    /**
     * `t`, where a number is needed: a type not yet known that may be any type becomes an
     * integer literal's, which settles to an integer or `field` type.
     */
    pub(crate) fn numeric(&mut self, t: TyId) -> TyId {
        if !matches!(self.kind(t), TyKind::Infer(_)) {
            return t;
        }
        let v = self.vars.fresh(&mut self.sema.types, false);
        self.unify(t, v);
        self.resolve(t)
    }
}
