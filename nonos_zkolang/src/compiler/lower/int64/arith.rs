/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * 64-bit addition and subtraction: the low halves with a carry or borrow into the high
 * ones. Unsigned, the high half must stay below `2^32`; signed, it wraps, and the result
 * overflows when the operands' signs allow no result of the sign it has.
 */

use super::super::cx::Lower;
use crate::compiler::ssa::V;

impl<'p> Lower<'p> {
    /** `a + b`: the low half and the high sum, below `2^33`. */
    pub(in crate::compiler::lower) fn add_raw(&mut self, a: (V, V), b: (V, V)) -> (V, V) {
        let s = self.b.add(a.0, b.0);
        let (lo, c) = self.carry(s);
        let h = self.b.add(a.1, b.1);
        (lo, self.b.add(h, c))
    }

    /** `a - b`: the low half and the high difference, in `(-2^32, 2^32)`. */
    pub(in crate::compiler::lower) fn sub_raw(&mut self, a: (V, V), b: (V, V)) -> (V, V) {
        let d = self.b.sub(a.0, b.0);
        let lift = self.b.konst(1i128 << 32);
        let d = self.b.add(d, lift);
        let (lo, no_borrow) = self.carry(d);
        let h = self.b.sub(a.1, b.1);
        let h = self.b.add(h, no_borrow);
        let one = self.one();
        (lo, self.b.sub(h, one))
    }

    /** `a + b` (`sub` false) or `a - b`, checked unless `wrap`. */
    pub(in crate::compiler::lower) fn add64(
        &mut self,
        a: (V, V),
        b: (V, V),
        sub: bool,
        signed: bool,
        wrap: bool,
    ) -> (V, V) {
        let (lo, h) = if sub {
            self.sub_raw(a, b)
        } else {
            self.add_raw(a, b)
        };
        let off = if sub { 1i128 << 32 } else { 0 };
        if !signed && !wrap {
            self.require_below(h, 32);
            return (lo, h);
        }
        let hi = self.wrap32(h, off);
        if signed && !wrap {
            let (sa, sb, sr) = (self.sign64(a.1), self.sign64(b.1), self.sign64(hi));
            let ab = self.xor(sa, sb);
            let ar = self.xor(sa, sr);
            let same = if sub { ab } else { self.b.not(ab) };
            let over = self.b.mul(same, ar);
            self.require_zero(over);
        }
        (lo, hi)
    }
}
