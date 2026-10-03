/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The operators on 64-bit integers, dispatched. */

use alloc::vec::Vec;

use super::super::cx::Lower;
use super::super::error::L;
use super::halves::halves;
use crate::compiler::source::Span;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a op b` on 64-bit integers of type `t`. */
    pub(in crate::compiler::lower) fn int64_op(
        &mut self,
        op: BinOp,
        a: &[V],
        b: &[V],
        t: IntTy,
        at: Span,
    ) -> L<Vec<V>> {
        let _ = at;
        let (x, y) = (halves(a), halves(b));
        let s = t.signed();
        let pair = |(lo, hi): (V, V)| alloc::vec![lo, hi];
        Ok(match op {
            BinOp::Add => pair(self.add64(x, y, false, s, false)),
            BinOp::Sub => pair(self.add64(x, y, true, s, false)),
            BinOp::Mul if s => pair(self.mul64_signed(x, y)),
            BinOp::Mul => pair(self.mul64_unsigned(x, y)),
            BinOp::Div | BinOp::Rem => pair(self.div64(op == BinOp::Div, x, y, t)),
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => {
                let lo = self.int_bitwise(op, x.0, y.0, IntTy::U32);
                let hi = self.int_bitwise(op, x.1, y.1, IntTy::U32);
                alloc::vec![lo, hi]
            }
            BinOp::Shl | BinOp::Shr => {
                let k = b.first().copied().unwrap_or(V(0));
                pair(self.shift64(op == BinOp::Shl, x, k, s))
            }
            BinOp::Lt => alloc::vec![self.less64(x, y, t)],
            BinOp::Gt => alloc::vec![self.less64(y, x, t)],
            BinOp::Le => {
                let gt = self.less64(y, x, t);
                alloc::vec![self.b.not(gt)]
            }
            _ => {
                let lt = self.less64(x, y, t);
                alloc::vec![self.b.not(lt)]
            }
        })
    }
}
