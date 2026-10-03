/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Operators on booleans and on field elements. */

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::ast::BinOp;

impl<'p> Lower<'p> {
    /** `a op b` on booleans; `false < true`. */
    pub(super) fn bool_op(&mut self, op: BinOp, a: V, b: V) -> V {
        let ab = self.and(a, b);
        let na = self.b.not(a);
        match op {
            BinOp::BitAnd => ab,
            BinOp::BitOr => {
                let s = self.b.add(a, b);
                self.b.sub(s, ab)
            }
            BinOp::BitXor => {
                let s = self.b.add(a, b);
                let two = self.b.add(ab, ab);
                self.b.sub(s, two)
            }
            BinOp::Lt => self.and(na, b),
            BinOp::Gt => {
                let nb = self.b.not(b);
                self.and(a, nb)
            }
            BinOp::Le => {
                let gt = self.bool_op(BinOp::Gt, a, b);
                self.b.not(gt)
            }
            _ => {
                let lt = self.bool_op(BinOp::Lt, a, b);
                self.b.not(lt)
            }
        }
    }

    /** `a op b` on `field` elements; `/` fails where it runs on a zero divisor. */
    pub(super) fn field_op(&mut self, op: BinOp, a: V, b: V) -> V {
        match op {
            BinOp::Add => self.b.add(a, b),
            BinOp::Sub => self.b.sub(a, b),
            BinOp::Mul => self.b.mul(a, b),
            _ => {
                let d = self.guarded(b, 1);
                let inv = self.b.emit(Inst::Inv(d));
                self.b.mul(a, inv)
            }
        }
    }
}
