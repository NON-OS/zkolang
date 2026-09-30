/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `x.pow(k)` (sections 7.1 and 7.2) by squaring. A `field` exponent is a constant. An
 * integer power is checked at each step it needs: a square `x^(2^j)` only where `k` has
 * a bit at `j` or above, since only then does `x^k` exceed it in size; so a power that
 * fits never fails for a square it does not use.
 */

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::interp::Interp;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::tir::TExpr;

impl<'p> Lower<'p> {
    /** `x.pow(k)` for `x` of kind `kind`, the expression `e`. */
    pub(super) fn pow(&mut self, x: V, k: Option<&TExpr>, kind: &TyKind, e: &TExpr) -> L<V> {
        let k = k.ok_or(LowerError::Unsupported(
            "a power without an exponent",
            e.span,
        ))?;
        let known = Interp::new(self.p, 1_000_000)
            .eval_const(k, 0)
            .ok()
            .map(|v| v.int());
        let TyKind::Int(t) = *kind else {
            let n = known.ok_or(LowerError::Unsupported(
                "a field power of a variable exponent",
                k.span,
            ))?;
            return Ok(self.power(x, n as u64, None));
        };
        let check = |lw: &mut Self, v: V, under: V| {
            let g = lw.g;
            lw.g = lw.and(g, under);
            lw.check_int(v, t);
            lw.g = g;
        };
        if let Some(n) = known {
            return Ok(self.power(x, n as u64, Some(t)));
        }
        let kv = self
            .expr(k)?
            .first()
            .copied()
            .ok_or(LowerError::Unsupported("an exponent", k.span))?;
        let kg = self.guarded(kv, 0);
        let bits: alloc::vec::Vec<V> = (0..32).map(|j| self.b.emit(Inst::Bit(kg, j, 32))).collect();
        let mut above = alloc::vec![self.b.konst(0); 33];
        for j in (0..32).rev() {
            let (a, b) = (above[j + 1], bits[j]);
            let ab = self.b.mul(a, b);
            let s = self.b.add(a, b);
            above[j] = self.b.sub(s, ab);
        }
        let (mut acc, mut sq) = (self.b.konst(1), x);
        for j in 0..32 {
            if j > 0 {
                sq = self.b.mul(sq, sq);
                check(self, sq, above[j]);
            }
            let one = self.b.konst(1);
            let factor = self.b.sel(bits[j], sq, one);
            acc = self.b.mul(acc, factor);
            check(self, acc, self.g);
        }
        Ok(acc)
    }
}
