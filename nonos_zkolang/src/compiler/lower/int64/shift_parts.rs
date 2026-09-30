/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The pieces of a 64-bit shift: a product split at bit 32, a quotient by a power of 2. */

use super::super::cx::Lower;
use crate::compiler::ssa::{Hint, Inst, V};

impl<'p> Lower<'p> {
    /**
     * The low 32 bits and the rest of `x * pow`, for `x < 2^32` and `pow <= 2^31` where the
     * point runs: advice `r < 2^32` and `q < 2^31` with `q 2^32 + r` equal to the product,
     * below `2^63 < p`, so they are its halves and the only ones.
     */
    pub(in crate::compiler::lower) fn split_product(&mut self, x: V, pow: V) -> (V, V) {
        let p = self.b.mul(x, pow);
        let p = self.guarded(p, 0);
        let base = self.b.konst(1i128 << 32);
        let q = self.b.emit(Inst::Advice(Hint::Quot(p, base)));
        let r = self.b.emit(Inst::Advice(Hint::Rem(p, base)));
        self.b.emit(Inst::RangeCheck(q, 31));
        self.b.emit(Inst::RangeCheck(r, 32));
        let qb = self.b.mul(q, base);
        let s = self.b.add(qb, r);
        let d = self.b.sub(s, p);
        self.b.emit(Inst::AssertZero(d));
        (r, q)
    }

    /** The quotient and remainder of `x < 2^32` by `pow`, a power of 2 below `2^32`. */
    pub(in crate::compiler::lower) fn divmod_pow(&mut self, x: V, pow: V) -> (V, V) {
        let xg = self.guarded(x, 0);
        let q = self.b.emit(Inst::Quot(xg, pow, 32));
        let r = self.b.emit(Inst::Rem(xg, pow, 32));
        (q, r)
    }

    /** The high half of `x + 2^63` for the high half `hi` of a signed pattern `x`. */
    pub(in crate::compiler::lower) fn flip_sign(&mut self, hi: V) -> V {
        let sign = self.sign64(hi);
        let half = self.b.konst(1i128 << 31);
        let up = self.b.add(hi, half);
        let top = self.b.konst(1i128 << 32);
        let st = self.b.mul(sign, top);
        self.b.sub(up, st)
    }
}
