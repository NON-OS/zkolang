/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A 64-bit power of a runtime exponent, a `u32`: its six low bits read from the top down,
 * squaring and then multiplying by `x` where a bit is set, so each value made is `x^e` for
 * some `e <= k`, which fits whenever `x^k` does, as for the 32-bit power (`lower/pow.rs`).
 */

use super::super::cx::Lower;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /**
     * `x^k` for the runtime exponent `k`. Only `k`'s six low bits are squared and multiplied
     * in: for `|x| >= 2` a power of `64` or more does not fit, so above them the run fails
     * unless `x` is `0`, `1` or `-1`, whose powers are `0`, `1` and the low bits' power.
     */
    pub(super) fn pow64_var(&mut self, x: (V, V), k: V, signed: bool) -> (V, V) {
        let kg = self.guarded(k, 0);
        let (zero, one) = (self.b.konst(0), self.one());
        let (mut acc, mut low) = ((one, zero), zero);
        for j in (0..6u8).rev() {
            let bit = self.b.emit(Inst::Bit(kg, j, 32));
            if j < 5 {
                acc = self.mul64(acc, acc, signed);
            }
            let factor = (self.b.sel(bit, x.0, one), self.b.sel(bit, x.1, zero));
            acc = self.mul64(acc, factor, signed);
            let place = self.b.konst(1i128 << j);
            let part = self.b.mul(bit, place);
            low = self.b.add(low, part);
        }
        let high = self.b.sub(kg, low);
        let small = self.b.emit(Inst::Eq(high, zero));
        let big = self.b.sub(one, small);
        let is = |lw: &mut Self, (lo, hi): (i128, i128)| {
            let (lo, hi) = (lw.b.konst(lo), lw.b.konst(hi));
            let (a, b) = (lw.b.emit(Inst::Eq(x.0, lo)), lw.b.emit(Inst::Eq(x.1, hi)));
            lw.b.mul(a, b)
        };
        let (is0, is1) = (is(self, (0, 0)), is(self, (1, 0)));
        let all = (1 << 32) - 1;
        let unit = match signed {
            true => is(self, (all, all)),
            false => zero,
        };
        let fits = self.b.add(is0, is1);
        let fits = self.b.add(fits, unit);
        let not_fits = self.b.sub(one, fits);
        let bad = self.b.mul(big, not_fits);
        self.require_zero(bad);
        let gone = self.b.mul(big, is0);
        (self.b.sel(gone, zero, acc.0), self.b.sel(gone, zero, acc.1))
    }
}
