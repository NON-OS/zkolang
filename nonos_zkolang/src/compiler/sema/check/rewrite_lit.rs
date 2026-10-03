/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Rewriting literals: `-` on a literal folds into a negative literal, and every integer
 * literal must fit its type (E0302).
 */

use nonos_stark::field::P;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::UnOp;
use crate::compiler::tir::{TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Fold `-` on an integer literal of a signed or `field` type into the literal. */
    pub(super) fn fold_negation(&mut self, e: &mut TExpr) {
        let TExprKind::Unary(UnOp::Neg, inner) = &e.kind else {
            return;
        };
        let negatable = match self.kind(e.ty) {
            TyKind::Int(i) => i.signed(),
            k => k == TyKind::Field,
        };
        if let (TExprKind::Lit(TLit::Int(v)), true) = (&inner.kind, negatable) {
            if let Some(n) = v.checked_neg() {
                e.kind = TExprKind::Lit(TLit::Int(n));
            }
        }
    }

    /**
     * The literal `v` of type `ty`, reported if it does not fit (E0302). A negative `field`
     * literal stands for `p - |v|`.
     */
    pub(super) fn check_fits(&mut self, v: i128, ty: TyId, at: Span) -> i128 {
        let p = i128::from(P);
        let (fits, v) = match self.kind(ty) {
            TyKind::Int(t) => (t.contains(v), v),
            TyKind::Field => (-p < v && v < p, v.rem_euclid(p)),
            _ => (true, v),
        };
        if !fits {
            let shown = self.show(ty);
            let d = Diagnostic::error(
                Code::LITERAL_OUT_OF_RANGE,
                alloc::format!("the literal does not fit `{shown}`"),
                at,
                alloc::format!("not a value of `{shown}`"),
            );
            self.sema.diags.push(d);
        }
        v
    }
}
