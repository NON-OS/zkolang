/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Signed 64-bit division: the magnitudes divided, the signs restored. */

use super::super::cx::Lower;
use crate::compiler::ssa::V;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a / b` when `div`, else `a % b`, for 64-bit integers of type `t`. */
    pub(in crate::compiler::lower) fn div64(
        &mut self,
        div: bool,
        a: (V, V),
        b: (V, V),
        t: IntTy,
    ) -> (V, V) {
        if !t.signed() {
            let (q, r) = self.div64_unsigned(a, b);
            return if div { q } else { r };
        }
        let (sa, sb) = (self.sign64(a.1), self.sign64(b.1));
        let (ma, mb) = (self.negate_if(sa, a), self.negate_if(sb, b));
        let (q, r) = self.div64_unsigned(ma, mb);
        let neg = self.xor(sa, sb);
        /* A positive quotient must be below 2^63: `MIN / -1` and `MIN % -1` fail. */
        let zero = self.b.konst(0);
        let high = self.b.sel(neg, zero, q.1);
        self.require_below(high, 31);
        if div {
            self.negate_if(neg, q)
        } else {
            self.negate_if(sa, r)
        }
    }
}
