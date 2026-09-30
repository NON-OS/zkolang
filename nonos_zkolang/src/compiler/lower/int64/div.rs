/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * 64-bit division: advice halves of the quotient `q` and remainder `r`, pinned whether
 * or not the point runs, since the operands are replaced by 0 and 1 where it does not
 * and the constraints are not conditioned. With `q1 b1 = 0`, the middle products below
 * `2^32`, `q0 b0 + r0` split into halves, and `r < b`, every sum stays below `p`, so
 * `a = q b + r` holds over the integers and `q`, `r` are unique. A zero divisor leaves
 * no `r < b`.
 */

use super::super::cx::Lower;
use crate::compiler::ssa::{Hint, Inst, V};
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** The quotient and remainder of unsigned `a / b`, failing where it runs if `b = 0`. */
    pub(in crate::compiler::lower) fn div64_unsigned(
        &mut self,
        a: (V, V),
        b: (V, V),
    ) -> ((V, V), (V, V)) {
        let a = (self.guarded(a.0, 0), self.guarded(a.1, 0));
        let b = (self.guarded(b.0, 1), self.guarded(b.1, 0));
        let g = self.g;
        self.g = self.one();
        let part = |lw: &mut Self, part| lw.b.emit(Inst::Advice(Hint::Div64 { a, b, part }));
        let (q0, q1, r0, r1) = (part(self, 0), part(self, 1), part(self, 2), part(self, 3));
        for x in [q0, q1, r0, r1] {
            self.b.emit(Inst::RangeCheck(x, 32));
        }
        let qb = self.b.mul(q1, b.1);
        self.b.emit(Inst::AssertZero(qb));
        let x = self.b.mul(q0, b.1);
        let y = self.b.mul(q1, b.0);
        let cross = self.b.add(x, y);
        self.b.emit(Inst::RangeCheck(cross, 32));
        let t0 = self.b.mul(q0, b.0);
        let t0 = self.b.add(t0, r0);
        let (l, h) = self.field_halves(t0);
        let d0 = self.b.sub(l, a.0);
        self.b.emit(Inst::AssertZero(d0));
        let t1 = self.b.add(cross, r1);
        let t1 = self.b.add(t1, h);
        let d1 = self.b.sub(t1, a.1);
        self.b.emit(Inst::AssertZero(d1));
        let lt = self.less64((r0, r1), b, IntTy::U64);
        let bad = self.b.not(lt);
        self.b.emit(Inst::AssertZero(bad));
        self.g = g;
        ((q0, q1), (r0, r1))
    }
}
