/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Integer division and remainder (section 7.2): the quotient truncated toward zero, the
 * remainder with the dividend's sign. A signed division divides the magnitudes, which are
 * at most `2^(N-1)`, and restores the signs; `MIN / -1` then gives `2^(N-1)`, which the
 * range check rejects, and `MIN % -1` fails with it.
 */

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a / b` when `div`, else `a % b`, for integers of type `t`. */
    pub(super) fn int_div(&mut self, div: bool, a: V, b: V, t: IntTy) -> V {
        let n = t.bits() as u8;
        let zero = self.b.konst(0);
        let is_zero = self.b.emit(Inst::Eq(b, zero));
        self.require_zero(is_zero);
        let (sa, sb) = (self.negative(a, t), self.negative(b, t));
        let (ma, mb) = (self.magnitude(a, sa), self.magnitude(b, sb));
        let (ma, mb) = (self.guarded(ma, 0), self.guarded(mb, 1));
        let q = self.b.emit(Inst::Quot(ma, mb, n));
        let r = self.b.emit(Inst::Rem(ma, mb, n));
        let s = self.b.add(sa, sb);
        let both = self.b.mul(sa, sb);
        let two = self.b.add(both, both);
        let flip = self.b.sub(s, two);
        let q = self.signed(q, flip);
        let r = self.signed(r, sa);
        self.check_int(q, t);
        if div {
            q
        } else {
            r
        }
    }

    /** `|x|`, `neg` saying whether `x` is negative. */
    fn magnitude(&mut self, x: V, neg: V) -> V {
        self.signed(x, neg)
    }

    /** `-x` when `neg`, else `x`. */
    fn signed(&mut self, x: V, neg: V) -> V {
        let zero = self.b.konst(0);
        let minus = self.b.sub(zero, x);
        self.b.sel(neg, minus, x)
    }
}
