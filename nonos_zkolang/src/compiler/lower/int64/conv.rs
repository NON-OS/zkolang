/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Conversions to and from 64-bit integers (sections 7.8 and 7.9). `checked_from` checks
 * the value against the target's range from the halves; `wrapping_from` keeps the low
 * bits of the pattern, extending a narrower signed value's sign.
 */

use alloc::vec::Vec;

use super::super::cx::Lower;
use super::super::error::{LowerError, L};
use super::halves::halves;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::V;
use crate::compiler::tir::{Builtin, TExpr};

impl<'p> Lower<'p> {
    /** A conversion `b` of `v` from `from` to `to`, one side 64 bits wide. */
    pub(in crate::compiler::lower) fn convert64(
        &mut self,
        b: Builtin,
        v: &[V],
        from: &TyKind,
        to: &TyKind,
        e: &TExpr,
    ) -> L<Vec<V>> {
        let wide = |k: &TyKind| matches!(k, TyKind::Int(i) if i.bits() > 32);
        let x = v.first().copied().unwrap_or(V(0));
        let zero = self.b.konst(0);
        match (b, wide(from), to) {
            (Builtin::FromLeBits, _, _) => {
                let (lo, hi) = self.join_bits64(v);
                Ok(alloc::vec![lo, hi])
            }
            (Builtin::CheckedFrom | Builtin::WrappingFrom, false, TyKind::Int(t)) => {
                let (lo, hi) = match from {
                    TyKind::Int(s) if b == Builtin::CheckedFrom && s.signed() && !t.signed() => {
                        self.require_below(x, 31);
                        (x, zero)
                    }
                    TyKind::Int(s) if s.signed() => {
                        let neg = self.negative(x, *s);
                        let top = self.b.konst(1i128 << 32);
                        let lift = self.b.mul(neg, top);
                        let ones = self.b.konst((1i128 << 32) - 1);
                        (self.b.add(x, lift), self.b.mul(neg, ones))
                    }
                    TyKind::Int(_) | TyKind::Bool => (x, zero),
                    _ => self.field_halves(x),
                };
                if b == Builtin::CheckedFrom && t.signed() && matches!(from, TyKind::Field) {
                    self.require_below(hi, 31);
                }
                Ok(alloc::vec![lo, hi])
            }
            (Builtin::CheckedFrom | Builtin::WrappingFrom, true, _) => {
                Ok(self.narrow64(b, halves(v), from, to))
            }
            _ => Err(LowerError::Unsupported(
                "this conversion of a 64-bit integer",
                e.span,
            )),
        }
    }
}
