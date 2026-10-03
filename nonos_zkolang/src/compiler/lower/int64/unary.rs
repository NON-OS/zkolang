/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Prefix operators on 64-bit integers. */

use alloc::vec::Vec;

use super::super::cx::Lower;
use super::super::error::L;
use super::halves::halves;
use crate::compiler::source::Span;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::UnOp;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `op a` on a 64-bit integer of type `t`. */
    pub(in crate::compiler::lower) fn int64_unary(
        &mut self,
        op: UnOp,
        a: &[V],
        t: IntTy,
        at: Span,
    ) -> L<Vec<V>> {
        let _ = at;
        let x = halves(a);
        Ok(match op {
            UnOp::Neg => {
                let zero = self.b.konst(0);
                let (lo, hi) = self.add64((zero, zero), x, true, t.signed(), false);
                alloc::vec![lo, hi]
            }
            UnOp::Not => {
                let all = self.b.konst((1i128 << 32) - 1);
                alloc::vec![self.b.sub(all, x.0), self.b.sub(all, x.1)]
            }
        })
    }
}
