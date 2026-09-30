/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bits (section 7.9): a value's pattern as bits, and bits back to a value. */

use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** `x.to_le_bits()`. */
    pub(super) fn bits_of(&mut self, x: V, from: &TyKind) -> Vec<V> {
        match from {
            TyKind::Int(t) => self.pattern_bits(x, *t),
            _ => self.low_bits(x, 64, 64),
        }
    }

    /** `T::from_le_bits(bits)`; a `field` fails, where it runs, on a non-canonical pattern. */
    pub(super) fn value_of_bits(&mut self, bits: &[V], to: &TyKind) -> V {
        if let TyKind::Int(t) = to {
            return self.pattern_value(bits, *t);
        }
        let (low, high) = bits.split_at(bits.len().min(32));
        let (lo, hi) = (self.bits_value(low), self.bits_value(high));
        let top = self.b.konst((1i128 << 32) - 1);
        let at_top = self.b.emit(Inst::Eq(hi, top));
        let both = self.b.mul(at_top, lo);
        self.require_zero(both);
        let shift = self.b.konst(1i128 << 32);
        let high = self.b.mul(hi, shift);
        self.b.add(lo, high)
    }
}
