/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Bitwise operators on integers, bit by bit on their patterns. */

use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::ssa::V;
use crate::compiler::syntax::ast::BinOp;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** `a op b` for `&`, `|` or `^` on integers of type `t`, bit by bit. */
    pub(super) fn int_bitwise(&mut self, op: BinOp, a: V, b: V, t: IntTy) -> V {
        let (xs, ys) = (self.pattern_bits(a, t), self.pattern_bits(b, t));
        let bits: Vec<V> = xs
            .iter()
            .zip(&ys)
            .map(|(&x, &y)| {
                let xy = self.b.mul(x, y);
                match op {
                    BinOp::BitAnd => xy,
                    BinOp::BitOr => {
                        let s = self.b.add(x, y);
                        self.b.sub(s, xy)
                    }
                    _ => {
                        let s = self.b.add(x, y);
                        let two = self.b.add(xy, xy);
                        self.b.sub(s, two)
                    }
                }
            })
            .collect();
        self.pattern_value(&bits, t)
    }
}
