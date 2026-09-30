/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Bit-level work on several integers at once. Their patterns' bits are read together from
 * the top down, each checked to be 0 or 1 and doubled into its integer's sum as it is read,
 * and bit `k` of the result is made from bit `k` of each and doubled into the result's sum.
 * So only the sums and one bit of each need registers at a time, where reading each
 * integer's bits in turn would keep all of them. Each sum must equal its (offset, guarded)
 * integer, which for fewer than 64 bits makes the bits exactly its bits.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::ssa::{Hint, Inst, V};
use crate::compiler::syntax::IntTy;

/** A running sum of bits read from the top, doubled at each. */
fn push(lw: &mut Lower<'_>, sum: Option<V>, b: V) -> V {
    match sum {
        Some(s) => {
            let twice = lw.b.add(s, s);
            lw.b.add(twice, b)
        }
        None => b,
    }
}

impl<'p> Lower<'p> {
    /** The integer of type `t` whose pattern's bit `k` is `f` of bit `k` of each of `xs`. */
    pub(super) fn lockstep(
        &mut self,
        xs: &[V],
        t: IntTy,
        f: &mut dyn FnMut(&mut Self, &[V]) -> V,
    ) -> V {
        let n = t.bits();
        let offs: Vec<V> = xs.iter().map(|&x| self.pattern_offset(x, t)).collect();
        let (mut sums, mut result, mut top) = (vec![None; xs.len()], None, None);
        for k in (0..n).rev() {
            let mut bits = Vec::with_capacity(xs.len());
            for (i, &o) in offs.iter().enumerate() {
                let b = self.b.emit(Inst::Advice(Hint::Bit(o, k as u8)));
                self.b.emit(Inst::AssertBool(b));
                sums[i] = Some(push(self, sums[i], b));
                let flip = t.signed() && k + 1 == n;
                bits.push(if flip { self.b.not(b) } else { b });
            }
            let r = f(self, &bits);
            top = top.or(Some(r));
            result = Some(push(self, result, r));
        }
        for (s, &o) in sums.iter().zip(&offs) {
            let d = self.b.sub(s.unwrap_or(o), o);
            self.b.emit(Inst::AssertZero(d));
        }
        let value = result.unwrap_or_else(|| self.b.konst(0));
        match (t.signed(), top) {
            (true, Some(top)) => {
                let w = self.b.konst(1i128 << n);
                let wt = self.b.mul(top, w);
                self.b.sub(value, wt)
            }
            _ => value,
        }
    }
}
