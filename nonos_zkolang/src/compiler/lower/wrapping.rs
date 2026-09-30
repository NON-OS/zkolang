/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Wrapping arithmetic (section 7.2): the result modulo `2^N`, read in two's complement if
 * signed. The exact result `x` plus a multiple `c` of `2^N` that makes it non-negative is
 * decomposed; its low `N` bits are the pattern of the wrapped value.
 */

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::Builtin;

impl<'p> Lower<'p> {
    /** `a.wrapping_*(b)` for integers of type `t`. */
    pub(super) fn wrapping(&mut self, op: Builtin, a: V, b: V, t: IntTy) -> V {
        let n = t.bits();
        let (x, c, width) = match op {
            Builtin::WrappingAdd if t.signed() => (self.b.add(a, b), 1i128 << n, n + 1),
            Builtin::WrappingAdd => (self.b.add(a, b), 0, n + 1),
            Builtin::WrappingSub => (self.b.sub(a, b), 1i128 << n, n + 1),
            Builtin::WrappingMul if t.signed() => (self.b.mul(a, b), 1i128 << (2 * n - 2), 2 * n),
            Builtin::WrappingMul => (self.b.mul(a, b), 0, 2 * n),
            _ => {
                let zero = self.b.konst(0);
                (self.b.sub(zero, a), 1i128 << n, n + 1)
            }
        };
        let c = self.b.konst(c);
        let u = self.b.add(x, c);
        let bits = self.low_bits(u, n, width);
        self.pattern_value(&bits, t)
    }

    /** The low `n` bits of `u`, where the point runs `0 <= u < 2^width`, `width <= 64`. */
    pub(super) fn low_bits(&mut self, u: V, n: u32, width: u32) -> alloc::vec::Vec<V> {
        let u = self.guarded(u, 0);
        (0..n)
            .map(|k| match width < 64 {
                true => self.b.emit(Inst::Bit(u, k as u8, width as u8)),
                false => self.b.emit(Inst::FieldBit(u, k as u8)),
            })
            .collect()
    }
}
