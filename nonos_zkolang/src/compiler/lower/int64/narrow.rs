/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Converting a 64-bit value to another type: checked against its range, or wrapped. */

use alloc::vec::Vec;

use super::super::cx::Lower;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::V;
use crate::compiler::tir::Builtin;

impl<'p> Lower<'p> {
    /** `b` of the 64-bit pattern `x`, of kind `from`, to `to`. */
    pub(in crate::compiler::lower) fn narrow64(
        &mut self,
        b: Builtin,
        x: (V, V),
        from: &TyKind,
        to: &TyKind,
    ) -> Vec<V> {
        let signed = matches!(from, TyKind::Int(s) if s.signed());
        let zero = self.b.konst(0);
        match (b, to) {
            (Builtin::WrappingFrom, TyKind::Int(t)) if t.bits() > 32 => alloc::vec![x.0, x.1],
            (Builtin::WrappingFrom, TyKind::Int(t)) => {
                let bits = self.low_bits(x.0, t.bits(), 32);
                alloc::vec![self.pattern_value(&bits, *t)]
            }
            (_, TyKind::Int(t)) if t.bits() > 32 => {
                if signed != t.signed() {
                    self.require_below(x.1, 31);
                }
                alloc::vec![x.0, x.1]
            }
            (_, TyKind::Int(t)) => {
                let n = t.bits();
                let s = if signed { self.sign64(x.1) } else { zero };
                let top = self.b.konst((1i128 << 32) - 1);
                let off = self.b.sub(x.1, top);
                let high = self.b.sel(s, off, x.1);
                self.require_zero(high);
                if !t.signed() {
                    self.require_zero(s);
                    self.require_below(x.0, n);
                    return alloc::vec![x.0];
                }
                let shift = self.b.konst((1i128 << 32) - (1i128 << (n - 1)));
                let low = self.b.sub(x.0, shift);
                let low = self.b.sel(s, low, x.0);
                self.require_below(low, n - 1);
                let wrap = self.b.konst(1i128 << 32);
                let neg = self.b.sub(x.0, wrap);
                alloc::vec![self.b.sel(s, neg, x.0)]
            }
            _ => {
                self.require_zero(x.1);
                self.require_below(x.0, 1);
                alloc::vec![self.guarded(x.0, 0)]
            }
        }
    }
}
