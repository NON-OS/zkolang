/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Integer comparison and sign, from the top bit of an offset difference. */

use super::cx::Lower;
use crate::compiler::ssa::V;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a < b` for integers of type `t`: the top bit of `a - b + 2^N`, offset if signed. */
    pub(super) fn less(&mut self, a: V, b: V, t: IntTy) -> V {
        let n = t.bits();
        let d = self.b.sub(a, b);
        let top = self.b.konst(1i128 << n);
        let d = self.b.add(d, top);
        let d = self.guarded(d, 1i128 << n);
        let ge = self
            .b
            .emit(crate::compiler::ssa::Inst::Bit(d, n as u8, (n + 1) as u8));
        self.b.not(ge)
    }

    /** 1 if the integer `x` of type `t` is negative, else 0. */
    pub(super) fn negative(&mut self, x: V, t: IntTy) -> V {
        if !t.signed() {
            return self.b.konst(0);
        }
        let n = t.bits();
        let half = self.b.konst(1i128 << (n - 1));
        let shifted = self.b.add(x, half);
        let shifted = self.guarded(shifted, 1i128 << (n - 1));
        let nonneg = self.b.emit(crate::compiler::ssa::Inst::Bit(
            shifted,
            (n - 1) as u8,
            n as u8,
        ));
        self.b.not(nonneg)
    }
}
