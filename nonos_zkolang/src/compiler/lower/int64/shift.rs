/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * 64-bit shifts by `k < 64` on the halves: by `s = k mod 32`, multiplying each half by
 * `2^s` and splitting the products at bit 32, or dividing each by `2^s`; then by 32 more,
 * moving the halves, if bit 5 of `k` is set. A signed `x >> k` is `(x + 2^63) >> k` less
 * `2^(63-k)`, as for narrower integers. No step holds more than a few values at once.
 */

use super::super::cx::Lower;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** `x << k` when `left`, else `x >> k`, arithmetic if `signed`. */
    pub(in crate::compiler::lower) fn shift64(
        &mut self,
        left: bool,
        x: (V, V),
        k: V,
        signed: bool,
    ) -> (V, V) {
        let last = self.b.konst(63);
        let room = self.b.sub(last, k);
        self.require_below(room, 6);
        let kg = self.count(k, 64);
        let mut pow = self.one();
        for j in 0..5u8 {
            let bit = self.b.emit(Inst::Bit(kg, j, 6));
            let step = self.b.konst((1i128 << (1u32 << j)) - 1);
            let scaled = self.b.mul(bit, step);
            let one = self.one();
            let factor = self.b.add(scaled, one);
            pow = self.b.mul(pow, factor);
        }
        let big = self.b.emit(Inst::Bit(kg, 5, 6));
        let zero = self.b.konst(0);
        if left {
            let (l_lo, l_hi) = self.split_product(x.0, pow);
            let (h_lo, _) = self.split_product(x.1, pow);
            let sum = self.b.add(h_lo, l_hi);
            let hi = self.carry(sum).0;
            return (self.b.sel(big, zero, l_lo), self.b.sel(big, l_lo, hi));
        }
        let hi = match signed {
            true => self.flip_sign(x.1),
            false => x.1,
        };
        let (ql, _) = self.divmod_pow(x.0, pow);
        let (qh, rh) = self.divmod_pow(hi, pow);
        let inv = self.b.emit(Inst::Inv(pow));
        let base = self.b.konst(1i128 << 32);
        let m = self.b.mul(base, inv);
        let moved = self.b.mul(rh, m);
        let low = self.b.add(ql, moved);
        let (lo, hi) = (self.b.sel(big, qh, low), self.b.sel(big, zero, qh));
        if !signed {
            return (lo, hi);
        }
        let half = self.b.konst(1i128 << 31);
        let c = self.b.mul(half, inv);
        let c = (self.b.sel(big, c, zero), self.b.sel(big, zero, c));
        let (lo, hd) = self.sub_raw((lo, hi), c);
        (lo, self.wrap32(hd, 1i128 << 32))
    }
}
