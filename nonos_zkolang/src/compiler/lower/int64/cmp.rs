/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * 64-bit comparison: the high halves decide, and the low ones when the high are equal. A
 * signed high half is read as a 32-bit signed integer.
 */

use super::super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a < b` for 64-bit integers of type `t`. */
    pub(in crate::compiler::lower) fn less64(&mut self, a: (V, V), b: (V, V), t: IntTy) -> V {
        let (ah, bh, ht) = match t.signed() {
            true => (self.signed_half(a.1), self.signed_half(b.1), IntTy::I32),
            false => (a.1, b.1, IntTy::U32),
        };
        let lt_hi = self.less(ah, bh, ht);
        let eq_hi = self.b.emit(Inst::Eq(a.1, b.1));
        let lt_lo = self.less(a.0, b.0, IntTy::U32);
        let low = self.b.mul(eq_hi, lt_lo);
        self.b.add(lt_hi, low)
    }

    /** The high half `h` of a signed pattern read as a signed 32-bit integer. */
    fn signed_half(&mut self, h: V) -> V {
        let s = self.sign64(h);
        let top = self.b.konst(1i128 << 32);
        let st = self.b.mul(s, top);
        self.b.sub(h, st)
    }
}
