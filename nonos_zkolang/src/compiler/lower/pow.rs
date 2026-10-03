/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `x.pow(k)` (sections 7.1 and 7.2) by squaring. A `field` exponent is a constant. A
 * variable integer exponent is read from its top bit down, squaring and then multiplying
 * by `x` where the bit is set, so each value made is `x^e` for some `e <= k`, and each is
 * checked. For `|x| >= 2` each such power is smaller in size than `x^k`, and for `|x| < 2`
 * each fits, so a power that fits never fails for one of them. Only the bits a power that
 * fits can use are read (`pow_var.rs`).
 */

use super::cx::Lower;
use super::error::{LowerError, L};
use crate::compiler::interp::Interp;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::V;
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
        if let Some(n) = known {
            return Ok(self.power(x, n as u64, Some(t)));
        }
        let kv = self
            .expr(k)?
            .first()
            .copied()
            .ok_or(LowerError::Unsupported("an exponent", k.span))?;
        Ok(self.pow_var(x, kv, t))
    }
}
