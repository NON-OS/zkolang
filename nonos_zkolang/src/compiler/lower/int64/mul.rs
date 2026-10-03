/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * 64-bit multiplication from 32-bit halves, each product of two below `p`. Unsigned and
 * checked, the product of the high halves must be 0 and the middle sum fit in 32 bits;
 * signed, the magnitudes multiply and the result's size is bounded by its sign. Wrapping
 * keeps the low 64 bits.
 */

use super::super::cx::Lower;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** The low and high halves of `a * b` for 32-bit halves `a` and `b`. */
    fn mul32(&mut self, a: V, b: V) -> (V, V) {
        let p = self.b.mul(a, b);
        self.field_halves(p)
    }

    /** `a * b` of unsigned 64-bit values, failing where it runs if it does not fit. */
    pub(in crate::compiler::lower) fn mul64_unsigned(&mut self, a: (V, V), b: (V, V)) -> (V, V) {
        let both_high = self.b.mul(a.1, b.1);
        self.require_zero(both_high);
        let (l0, h0) = self.mul32(a.0, b.0);
        let x = self.b.mul(a.0, b.1);
        let y = self.b.mul(a.1, b.0);
        let mid = self.b.add(x, y);
        let mid = self.b.add(mid, h0);
        self.require_below(mid, 32);
        (l0, mid)
    }

    /** `a * b` of signed 64-bit values, failing where it runs if it does not fit. */
    pub(in crate::compiler::lower) fn mul64_signed(&mut self, a: (V, V), b: (V, V)) -> (V, V) {
        let (sa, sb) = (self.sign64(a.1), self.sign64(b.1));
        let (ma, mb) = (self.negate_if(sa, a), self.negate_if(sb, b));
        let prod = self.mul64_unsigned(ma, mb);
        let neg = self.xor(sa, sb);
        let zero = self.b.konst(0);
        let low_zero = self.b.emit(Inst::Eq(prod.0, zero));
        let extra = self.b.mul(neg, low_zero);
        let bound = self.b.konst((1i128 << 31) - 1);
        let bound = self.b.add(bound, extra);
        let room = self.b.sub(bound, prod.1);
        self.require_below(room, 32);
        self.negate_if(neg, prod)
    }

    /** The low 64 bits of `a * b`. */
    pub(in crate::compiler::lower) fn mul64_wrapping(&mut self, a: (V, V), b: (V, V)) -> (V, V) {
        let (l0, h0) = self.mul32(a.0, b.0);
        let (c1, _) = self.mul32(a.0, b.1);
        let (c2, _) = self.mul32(a.1, b.0);
        let s = self.b.add(c1, c2);
        let s = self.b.add(s, h0);
        let bits = self.low_bits(s, 32, 34);
        (l0, self.bits_value(&bits))
    }
}
