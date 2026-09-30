/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Prefix operators: `-x` on `field` and signed integers, which fails for `MIN`; `!x` on
 * `bool`, and on integers the complement `2^N - 1 - x`, or `-1 - x` if signed.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::UnOp;
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** `op a`, the expression `e`. */
    pub(super) fn unary(&mut self, op: UnOp, a: &TExpr, e: &TExpr) -> L<Vec<V>> {
        let v = self.expr(a)?;
        let kind = self.p.types.kind(e.ty).clone();
        if let TyKind::Int(t) = kind {
            if t.bits() > 32 {
                return self.int64_unary(op, &v, t, e.span);
            }
        }
        let Some(&x) = v.first() else {
            return Err(LowerError::Unsupported(
                "an operator on an empty value",
                e.span,
            ));
        };
        let zero = self.b.konst(0);
        let out = match (op, kind) {
            (UnOp::Neg, TyKind::Int(t)) => {
                let r = self.b.sub(zero, x);
                self.check_int(r, t);
                r
            }
            (UnOp::Neg, _) => self.b.sub(zero, x),
            (UnOp::Not, TyKind::Int(t)) => {
                let all = if t.signed() {
                    -1
                } else {
                    (1i128 << t.bits()) - 1
                };
                let all = self.b.konst(all);
                self.b.sub(all, x)
            }
            (UnOp::Not, _) => self.b.not(x),
        };
        Ok(alloc::vec![out])
    }
}
