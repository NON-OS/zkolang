/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A power of an integer of at most 32 bits with a runtime exponent. For `|x| >= 2`, `x^k`
 * fits `N` bits only for `k < N`, so only the exponent's `log2 N` low bits are squared and
 * multiplied in, each product checked; above them the run fails unless `x` is `0`, `1` or,
 * signed, `-1`, whose powers are `0`, `1` and the low bits' power, `N` being even.
 */

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `x^k` for `x` of type `t` and the runtime exponent `k`, a `u32`. */
    pub(super) fn pow_var(&mut self, x: V, k: V, t: IntTy) -> V {
        let kg = self.guarded(k, 0);
        let (zero, one) = (self.b.konst(0), self.b.konst(1));
        let bits = u8::try_from(t.bits().trailing_zeros()).unwrap_or(5);
        let (mut acc, mut low) = (one, zero);
        for j in (0..bits).rev() {
            let bit = self.b.emit(Inst::Bit(kg, j, 32));
            if j + 1 < bits {
                acc = self.b.mul(acc, acc);
                self.check_int(acc, t);
            }
            let factor = self.b.sel(bit, x, one);
            acc = self.b.mul(acc, factor);
            self.check_int(acc, t);
            let place = self.b.konst(1i128 << j);
            let part = self.b.mul(bit, place);
            low = self.b.add(low, part);
        }
        let high = self.b.sub(kg, low);
        let small = self.b.emit(Inst::Eq(high, zero));
        let big = self.b.sub(one, small);
        let is0 = self.b.emit(Inst::Eq(x, zero));
        let is1 = self.b.emit(Inst::Eq(x, one));
        let fits = self.b.add(is0, is1);
        let fits = match t.signed() {
            true => {
                let minus = self.b.konst(-1);
                let unit = self.b.emit(Inst::Eq(x, minus));
                self.b.add(fits, unit)
            }
            false => fits,
        };
        let not_fits = self.b.sub(one, fits);
        let bad = self.b.mul(big, not_fits);
        self.require_zero(bad);
        let gone = self.b.mul(big, is0);
        self.b.sel(gone, zero, acc)
    }
}
