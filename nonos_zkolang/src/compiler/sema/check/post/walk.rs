/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checks of a settled body that evaluate constants: `for` bounds are constant (section
 * 8.6), a constant index is in bounds (section 7.6), a constant shift count is below the
 * width (section 7.5), a `field` exponent is constant (section 7.1), and the cost
 * warnings (section 15.3).
 */

use super::super::cx::FnCx;
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::tir::{Builtin, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Run the checks on `e` and everything in it. */
    pub(crate) fn post(&mut self, e: &TExpr) {
        match &e.kind {
            TExprKind::ForRange {
                lo, hi, inclusive, ..
            } => {
                for b in [lo, hi] {
                    if let Some(at) = self.sema.not_const(b) {
                        self.not_constant(at, "a `for` range bound is a constant expression");
                    }
                }
                self.post_range((lo, hi), *inclusive, e.span);
            }
            TExprKind::ForArray { array, .. } => {
                if let TyKind::Array(_, n) = self.kind(array.ty) {
                    self.post_unroll(u64::from(n), e.span);
                }
            }
            TExprKind::While { limit, .. } => self.post_unroll(u64::from(*limit), e.span),
            TExprKind::Index(a, i) => {
                self.post_index(a, i);
                self.post_dyn(a.ty, i);
            }
            TExprKind::Assign { place, .. } => self.post_place(place),
            TExprKind::Chain(first, links) => {
                for (op, l) in links {
                    self.post_shift(*op, first.ty, l);
                }
            }
            TExprKind::Builtin(Builtin::Pow, args) => {
                if let (Some(recv), Some(k)) = (args.first(), args.get(1)) {
                    if self.kind(recv.ty) == TyKind::Field {
                        if let Some(at) = self.sema.not_const(k) {
                            self.not_constant(at, "a `field` exponent is a constant expression");
                        }
                    }
                }
            }
            _ => {}
        }
        e.each_child(&mut |c| self.post(c));
    }

    /** The value of `e` if it is a constant expression that evaluates. */
    pub(super) fn constant(&mut self, e: &TExpr) -> Option<Value> {
        self.sema.not_const(e).is_none().then_some(())?;
        self.sema.const_eval(e, 0)
    }
}
