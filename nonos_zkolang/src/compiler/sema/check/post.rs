/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checks of a settled body that evaluate constants: `for` bounds are constant (section
 * 8.6), a constant index is in bounds (section 7.6), a constant shift count is below the
 * width (section 7.5), and a `field` exponent is constant (section 7.1).
 */

use super::cx::FnCx;
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::tir::{Builtin, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Run the checks on `e` and everything in it. */
    pub(crate) fn post(&mut self, e: &TExpr) {
        match &e.kind {
            TExprKind::ForRange { lo, hi, .. } => {
                for b in [lo, hi] {
                    if let Some(at) = self.sema.not_const(b) {
                        self.not_constant(at, "a `for` range bound is a constant expression");
                    }
                }
            }
            TExprKind::Index(a, i) => self.post_index(a, i),
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
