/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A 64-bit power by squaring, each step checked. */

use super::super::cx::Lower;
use crate::compiler::ssa::V;

impl<'p> Lower<'p> {
    /** `x^n` by squaring, each square and product checked. */
    pub(super) fn pow64(&mut self, x: (V, V), n: u64, signed: bool) -> (V, V) {
        let (zero, one) = (self.b.konst(0), self.one());
        let (mut acc, mut sq, mut n) = ((one, zero), x, n);
        let mul = |lw: &mut Self, a, b| {
            if signed {
                lw.mul64_signed(a, b)
            } else {
                lw.mul64_unsigned(a, b)
            }
        };
        while n > 0 {
            if n & 1 == 1 {
                acc = mul(self, acc, sq);
            }
            n >>= 1;
            if n > 0 {
                sq = mul(self, sq, sq);
            }
        }
        acc
    }
}
