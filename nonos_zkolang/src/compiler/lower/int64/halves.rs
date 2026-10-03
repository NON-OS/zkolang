/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The halves of 64-bit values: carries, signs, and the halves of a product or element. */

use super::super::cx::Lower;
use crate::compiler::ssa::{Inst, V};

/** The two halves of a 64-bit value's slots. */
pub(in crate::compiler::lower) fn halves(v: &[V]) -> (V, V) {
    (
        v.first().copied().unwrap_or(V(0)),
        v.get(1).copied().unwrap_or(V(0)),
    )
}

impl<'p> Lower<'p> {
    /** `s mod 2^32` and `s >> 32`, where the point runs `0 <= s < 2^33`. */
    pub(in crate::compiler::lower) fn carry(&mut self, s: V) -> (V, V) {
        let sg = self.guarded(s, 0);
        let c = self.b.emit(Inst::Bit(sg, 32, 33));
        let top = self.b.konst(1i128 << 32);
        let ct = self.b.mul(c, top);
        (self.b.sub(s, ct), c)
    }

    /** `(x + off) mod 2^32`, where the point runs `0 <= x + off < 2^33`. */
    pub(in crate::compiler::lower) fn wrap32(&mut self, x: V, off: i128) -> V {
        let o = self.b.konst(off);
        let u = self.b.add(x, o);
        self.carry(u).0
    }

    /** The sign bit of a 64-bit pattern whose high half is `hi`. */
    pub(in crate::compiler::lower) fn sign64(&mut self, hi: V) -> V {
        let hg = self.guarded(hi, 0);
        self.b.emit(Inst::Bit(hg, 31, 32))
    }

    /** The low and high 32 bits of the canonical representative of `x`. */
    pub(in crate::compiler::lower) fn field_halves(&mut self, x: V) -> (V, V) {
        let bits = self.low_bits(x, 64, 64);
        let (low, high) = bits.split_at(32);
        (self.bits_value(low), self.bits_value(high))
    }

    /** `a xor b` for booleans. */
    pub(in crate::compiler::lower) fn xor(&mut self, a: V, b: V) -> V {
        let s = self.b.add(a, b);
        let ab = self.b.mul(a, b);
        let two = self.b.add(ab, ab);
        self.b.sub(s, two)
    }

    /** The pattern of `-x mod 2^64` for the pattern `x`. */
    pub(in crate::compiler::lower) fn neg64(&mut self, x: (V, V)) -> (V, V) {
        let zero = self.b.konst(0);
        let (lo, hd) = self.sub_raw((zero, zero), x);
        (lo, self.wrap32(hd, 1i128 << 32))
    }

    /** `x` if `c` is 0, else the pattern of `-x`. */
    pub(in crate::compiler::lower) fn negate_if(&mut self, c: V, x: (V, V)) -> (V, V) {
        let n = self.neg64(x);
        (self.b.sel(c, n.0, x.0), self.b.sel(c, n.1, x.1))
    }
}
