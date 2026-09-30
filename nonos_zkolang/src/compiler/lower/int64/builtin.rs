/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The built-in methods with a 64-bit source or target. */

use alloc::vec::Vec;

use super::super::cx::Lower;
use super::super::error::{LowerError, L};
use super::halves::halves;
use crate::compiler::interp::Interp;
use crate::compiler::sema::ty::TyKind;
use crate::compiler::ssa::V;
use crate::compiler::tir::{Builtin, TExpr};

impl<'p> Lower<'p> {
    /** The built-in `b` on `vals`, the values of `args`, from and to the kinds `k`. */
    pub(in crate::compiler::lower) fn int64_builtin(
        &mut self,
        b: Builtin,
        vals: &[Vec<V>],
        args: &[TExpr],
        k: (&TyKind, &TyKind),
        e: &TExpr,
    ) -> L<Vec<V>> {
        let (from, to) = k;
        let first = vals.first().map_or(&[][..], |v| v);
        let x = halves(first);
        let y = halves(vals.get(1).map_or(&[][..], |v| v));
        let TyKind::Int(t) = *from else {
            return self.convert64(b, first, from, to, e);
        };
        if t.bits() <= 32
            || !matches!(to, TyKind::Int(u) if u.bits() > 32) && !matches!(b, Builtin::ToLeBits)
        {
            return self.convert64(b, first, from, to, e);
        }
        let pair = |(lo, hi): (V, V)| alloc::vec![lo, hi];
        Ok(match b {
            Builtin::WrappingAdd => pair(self.add64(x, y, false, t.signed(), true)),
            Builtin::WrappingSub => pair(self.add64(x, y, true, t.signed(), true)),
            Builtin::WrappingMul => pair(self.mul64_wrapping(x, y)),
            Builtin::WrappingNeg => pair(self.neg64(x)),
            Builtin::Min | Builtin::Max => {
                let lt = self.less64(x, y, t);
                let (a, c) = if b == Builtin::Min { (x, y) } else { (y, x) };
                alloc::vec![self.b.sel(lt, a.0, c.0), self.b.sel(lt, a.1, c.1)]
            }
            Builtin::ToLeBits => self.bits64(x),
            Builtin::Pow => {
                let exp = args
                    .get(1)
                    .and_then(|k| Interp::new(self.p, 1_000_000).eval_const(k, 0).ok());
                let n = exp.map(|v| v.int() as u64).ok_or(LowerError::Unsupported(
                    "a 64-bit power of a variable exponent",
                    e.span,
                ))?;
                pair(self.pow64(x, n, t.signed()))
            }
            _ => return self.convert64(b, first, from, to, e),
        })
    }
}
