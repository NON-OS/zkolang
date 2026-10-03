/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Integers of at most 32 bits (section 7.2). A value of `uN` is its slot `x`, of `iN` the
 * slot `x mod p`; every result is checked to be a value of its type where it is computed.
 * Sums and products of such values stay below `p / 2` in size, so the check is exact.
 */

use super::cx::Lower;
use super::error::L;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** Fail, where the point runs, unless `v` is a value of `t`. */
    pub(super) fn check_int(&mut self, v: V, t: IntTy) {
        let n = t.bits();
        if t.signed() {
            let half = self.b.konst(1i128 << (n - 1));
            let shifted = self.b.add(v, half);
            self.require_below(shifted, n);
        } else {
            self.require_below(v, n);
        }
    }

    /** `a op b` on integers of type `t`, of at most 32 bits. */
    pub(super) fn int_op(&mut self, op: BinOp, a: V, b: V, t: IntTy) -> L<V> {
        let v = match op {
            BinOp::Add => self.b.add(a, b),
            BinOp::Sub => self.b.sub(a, b),
            BinOp::Mul => self.b.mul(a, b),
            BinOp::Div | BinOp::Rem => return Ok(self.int_div(op == BinOp::Div, a, b, t)),
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => {
                return Ok(self.int_bitwise(op, a, b, t))
            }
            BinOp::Shl | BinOp::Shr => return Ok(self.shift(op == BinOp::Shl, a, b, t)),
            BinOp::Lt => return Ok(self.less(a, b, t)),
            BinOp::Gt => return Ok(self.less(b, a, t)),
            BinOp::Le => {
                let gt = self.less(b, a, t);
                return Ok(self.b.not(gt));
            }
            _ => {
                let lt = self.less(a, b, t);
                return Ok(self.b.not(lt));
            }
        };
        self.check_int(v, t);
        Ok(v)
    }
}
