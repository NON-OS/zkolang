/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A 64-bit power of a constant exponent by squaring, each step checked, and the checked
 * product the powers share.
 */

use super::super::cx::Lower;
use crate::compiler::ssa::V;

impl<'p> Lower<'p> {
    /** `x^n` by squaring, each square and product checked. */
    pub(super) fn pow64(&mut self, x: (V, V), n: u64, signed: bool) -> (V, V) {
        let (zero, one) = (self.b.konst(0), self.one());
        let (mut acc, mut sq, mut n) = ((one, zero), x, n);
        while n > 0 {
            if n & 1 == 1 {
                acc = self.mul64(acc, sq, signed);
            }
            n >>= 1;
            if n > 0 {
                sq = self.mul64(sq, sq, signed);
            }
        }
        acc
    }

    /** `a * b`, signed or not, failing where it runs if it does not fit. */
    pub(super) fn mul64(&mut self, a: (V, V), b: (V, V), signed: bool) -> (V, V) {
        match signed {
            true => self.mul64_signed(a, b),
            false => self.mul64_unsigned(a, b),
        }
    }
}
