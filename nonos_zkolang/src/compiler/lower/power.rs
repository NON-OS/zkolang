/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A power by squaring, checked at each step when it is an integer power. */

use super::cx::Lower;
use crate::compiler::ssa::V;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `x^n` by squaring, each square and product checked to be a value of `t` if given. */
    pub(super) fn power(&mut self, x: V, n: u64, t: Option<IntTy>) -> V {
        let (mut acc, mut sq, mut n) = (self.b.konst(1), x, n);
        while n > 0 {
            if n & 1 == 1 {
                acc = self.b.mul(acc, sq);
                if let Some(t) = t {
                    self.check_int(acc, t);
                }
            }
            n >>= 1;
            if n > 0 {
                sq = self.b.mul(sq, sq);
                if let Some(t) = t {
                    self.check_int(sq, t);
                }
            }
        }
        acc
    }
}
