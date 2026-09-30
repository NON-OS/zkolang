/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A constant index must be in bounds, and a constant shift count below the width. */

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `a[i]`: a constant `i` is below the length (E0309). */
    pub(super) fn post_index(&mut self, a: &TExpr, i: &TExpr) {
        let (TyKind::Array(_, n), Some(v)) = (self.kind(a.ty), self.constant(i)) else {
            return;
        };
        if v.int() >= i128::from(n) {
            let what = alloc::format!("index {} is out of bounds for an array of {n}", v.int());
            let d = Diagnostic::error(Code::INDEX_OUT_OF_BOUNDS, what, i.span, "out of bounds");
            self.sema.diags.push(d);
        }
    }

    /** `x op l` with `x` of type `ty`: a constant shift `l` is below the width (E0312). */
    pub(super) fn post_shift(&mut self, op: BinOp, ty: TyId, l: &TExpr) {
        if !matches!(op, BinOp::Shl | BinOp::Shr) {
            return;
        }
        let (TyKind::Int(t), Some(v)) = (self.kind(ty), self.constant(l)) else {
            return;
        };
        if v.int() >= i128::from(t.bits()) {
            let what = alloc::format!("a shift by {} of a value of {} bits", v.int(), t.bits());
            let d = Diagnostic::error(Code::SHIFT_TOO_FAR, what, l.span, "at least the width");
            self.sema.diags.push(d);
        }
    }
}
