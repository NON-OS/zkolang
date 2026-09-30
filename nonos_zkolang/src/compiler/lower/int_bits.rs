/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Integers as bits (section 7.5): the two's-complement pattern of an `N`-bit integer, and
 * back. A signed value `x` is decomposed as `x + 2^(N-1)`, whose bits are the pattern's
 * with the top one flipped, and a signed pattern weighs its top bit `-2^(N-1)`.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** The `N` bits of the pattern of `x`, of type `t`, least significant first. */
    pub(super) fn pattern_bits(&mut self, x: V, t: IntTy) -> Vec<V> {
        let n = t.bits();
        let off = match t.signed() {
            true => {
                let half = self.b.konst(1i128 << (n - 1));
                self.b.add(x, half)
            }
            false => x,
        };
        let off = self.guarded(off, 0);
        let mut bits: Vec<V> = (0..n)
            .map(|k| self.b.emit(Inst::Bit(off, k as u8, n as u8)))
            .collect();
        if t.signed() {
            if let Some(top) = bits.last_mut() {
                *top = self.b.not(*top);
            }
        }
        bits
    }

    /** The integer of type `t` whose pattern has the bits `bits`, least significant first. */
    pub(super) fn pattern_value(&mut self, bits: &[V], t: IntTy) -> V {
        let Some((&top, rest)) = bits.split_last() else {
            return self.b.konst(0);
        };
        let low = self.bits_value(rest);
        let w = 1i128 << rest.len();
        let weight = self.b.konst(if t.signed() { -w } else { w });
        let top = self.b.mul(top, weight);
        self.b.add(low, top)
    }

    /** `Σ 2^k bits[k]`, by doubling from the top bit down. */
    pub(super) fn bits_value(&mut self, bits: &[V]) -> V {
        let mut acc = self.b.konst(0);
        for &b in bits.iter().rev() {
            let twice = self.b.add(acc, acc);
            acc = self.b.add(twice, b);
        }
        acc
    }
}
