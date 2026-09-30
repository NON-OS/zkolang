/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Unsigned division of `n`-bit integers, `n <= 32`: advice `q` and `r` with `a = q b + r`,
 * all of `a`, `b`, `q`, `r` and `b - r - 1` below `2^n`. Then `r < b`, so `b` is not zero,
 * and `q b + r < 2^64 - 2^32 < p`, so the equation holds over the integers: `q` and `r` are
 * the quotient and remainder, the only ones.
 */

use super::rewrite::Rebuild;
use crate::compiler::ssa::{Hint, Inst, V};

impl Rebuild {
    /** The quotient and remainder of `a / b`. */
    pub(super) fn divide(&mut self, a: V, b: V, n: u8) -> (V, V) {
        if let Some(&qr) = self.div.get(&(a, b, n)) {
            return qr;
        }
        let q = self.b.emit(Inst::Advice(Hint::Quot(a, b)));
        let r = self.b.emit(Inst::Advice(Hint::Rem(a, b)));
        if n > 32 {
            /* Outside the gadget's range: the semantics fails, and so does this. */
            let one = self.b.konst(1);
            self.b.emit(Inst::AssertZero(one));
            return (q, r);
        }
        for x in [a, b, q, r] {
            self.b.emit(Inst::RangeCheck(x, n));
        }
        let one = self.b.konst(1);
        let gap = self.b.sub(b, r);
        let gap = self.b.sub(gap, one);
        self.b.emit(Inst::RangeCheck(gap, n));
        let qb = self.b.mul(q, b);
        let sum = self.b.add(qb, r);
        let diff = self.b.sub(sum, a);
        self.b.emit(Inst::AssertZero(diff));
        self.div.insert((a, b, n), (q, r));
        (q, r)
    }
}
