/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bitwise operators on integers, bit by bit on their patterns. */

use super::cx::Lower;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a op b` for `&`, `|` or `^` on integers of type `t`, bit by bit. */
    pub(super) fn int_bitwise(&mut self, op: BinOp, a: V, b: V, t: IntTy) -> V {
        self.lockstep(&[a, b], t, &mut |lw, bits| {
            let (x, y) = (bits[0], bits[1]);
            let xy = lw.b.mul(x, y);
            match op {
                BinOp::BitAnd => xy,
                BinOp::BitOr => {
                    let s = lw.b.add(x, y);
                    lw.b.sub(s, xy)
                }
                _ => {
                    let s = lw.b.add(x, y);
                    let two = lw.b.add(xy, xy);
                    lw.b.sub(s, two)
                }
            }
        })
    }
}
