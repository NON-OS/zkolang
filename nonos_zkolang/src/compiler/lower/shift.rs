/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Shifts (section 7.5) of an `N`-bit integer by a `u32` count `k`, which must be below
 * `N`. `x << k` is the low `N` bits of `pattern(x) * 2^k`, below `2^63`; `x >> k` is
 * `x / 2^k` rounded down, which for a signed `x` is `(x + 2^(N-1)) / 2^k - 2^(N-1-k)`.
 * `2^k` is the product over the bits of `k` of `2^(2^j)` or 1.
 */

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `x << k` when `left`, else `x >> k`, for `x` of type `t`. */
    pub(super) fn shift(&mut self, left: bool, x: V, k: V, t: IntTy) -> V {
        let n = t.bits();
        let log = n.trailing_zeros();
        let last = self.b.konst(i128::from(n - 1));
        let room = self.b.sub(last, k);
        self.require_below(room, log);
        let kg = self.count(k, u64::from(n));
        let mut pow = self.b.konst(1);
        for j in 0..log {
            let bit = self.b.emit(Inst::Bit(kg, j as u8, log as u8));
            let step = self.b.konst((1i128 << (1u32 << j)) - 1);
            let scaled = self.b.mul(bit, step);
            let one = self.one();
            let factor = self.b.add(scaled, one);
            pow = self.b.mul(pow, factor);
        }
        if left {
            let bits = self.pattern_bits(x, t);
            let pattern = self.bits_value(&bits);
            let product = self.b.mul(pattern, pow);
            let product = self.guarded(product, 0);
            let low: alloc::vec::Vec<V> = (0..n)
                .map(|b| self.b.emit(Inst::Bit(product, b as u8, 63)))
                .collect();
            return self.pattern_value(&low, t);
        }
        let half = 1i128 << (n - 1);
        let off = match t.signed() {
            true => {
                let h = self.b.konst(half);
                self.b.add(x, h)
            }
            false => x,
        };
        let off = self.guarded(off, 0);
        let q = self.b.emit(Inst::Quot(off, pow, n as u8));
        if !t.signed() {
            return q;
        }
        let inv = self.b.emit(Inst::Inv(pow));
        let h = self.b.konst(half);
        let back = self.b.mul(h, inv);
        self.b.sub(q, back)
    }

    /** The constant 1. */
    pub(super) fn one(&mut self) -> V {
        self.b.konst(1)
    }
}
