/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `as` (section 7.8), which the checker allows only where every value fits: a `bool` is
 * its 0 or 1, a narrower integer keeps its value, and an integer as `field` is `x mod p`.
 * Only a 64-bit side changes the slots: a signed value widened to 64 bits extends its sign.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** `a as T`, the expression `e` of type `T`. */
    pub(super) fn cast(&mut self, a: &TExpr, e: &TExpr) -> L<Vec<V>> {
        let v = self.expr(a)?;
        let types = &self.p.types;
        let (from, to) = (types.kind(a.ty).clone(), types.kind(e.ty).clone());
        let wide = |k: &TyKind| matches!(k, TyKind::Int(i) if i.bits() > 32);
        let Some(&x) = v.first() else {
            return Ok(v);
        };
        let zero = self.b.konst(0);
        Ok(match (from, to) {
            _ if a.ty == e.ty => v,
            (TyKind::Bool, t) if wide(&t) => alloc::vec![x, zero],
            (TyKind::Int(s), t) if !wide(&TyKind::Int(s)) && wide(&t) => {
                let neg = self.negative(x, s);
                let top = self.b.konst(1i128 << 32);
                let lift = self.b.mul(neg, top);
                let lo = self.b.add(x, lift);
                let ones = self.b.konst((1i128 << 32) - 1);
                alloc::vec![lo, self.b.mul(neg, ones)]
            }
            (TyKind::Int(s), TyKind::Field) if wide(&TyKind::Int(s)) => {
                let hi = v.get(1).copied().unwrap_or(zero);
                let top = self.b.konst(1i128 << 32);
                let high = self.b.mul(hi, top);
                let value = self.b.add(x, high);
                if !s.signed() {
                    return Ok(alloc::vec![value]);
                }
                let hg = self.guarded(hi, 0);
                let sign = self.b.emit(Inst::Bit(hg, 31, 32));
                let wrap = self.b.konst(1i128 << 64);
                let back = self.b.mul(sign, wrap);
                alloc::vec![self.b.sub(value, back)]
            }
            (_, t) if wide(&t) => {
                return Err(LowerError::Unsupported(
                    "this conversion to a 64-bit integer",
                    e.span,
                ))
            }
            _ => v,
        })
    }
}
