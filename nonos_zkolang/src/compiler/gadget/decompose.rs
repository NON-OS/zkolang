/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Bit decomposition: `n` advice bits, each constrained to 0 or 1, whose sum `Σ 2^k b_k`
 * must equal `v`. For `n < 64` the sum is below `2^63 < p`, so it equals `v` as an
 * integer: the bits exist exactly when `v < 2^n`, and they are unique. Rows: four per bit.
 */

use alloc::vec::Vec;

use super::rewrite::Rebuild;
use crate::compiler::ssa::{Hint, Inst, V};

impl Rebuild {
    /** The bits of `v`, least significant first, which must make up `v < 2^n`. */
    pub(super) fn decompose(&mut self, v: V, n: u8) -> Vec<V> {
        if let Some(bits) = self.bits.get(&(v, n)) {
            return bits.clone();
        }
        if n >= 64 {
            /* Outside the gadget's range: the semantics fails, and so does this. */
            let one = self.b.konst(1);
            self.b.emit(Inst::AssertZero(one));
            return Vec::new();
        }
        let bits = self.advice_bits(v, 0..n);
        let diff = match self.horner(&bits) {
            Some(sum) => self.b.sub(sum, v),
            None => v,
        };
        self.b.emit(Inst::AssertZero(diff));
        self.bits.insert((v, n), bits.clone());
        bits
    }

    /** The advice bits `ks` of `v`, each constrained to 0 or 1. */
    pub(super) fn advice_bits(&mut self, v: V, ks: core::ops::Range<u8>) -> Vec<V> {
        let mut bits = Vec::with_capacity(ks.len());
        for k in ks {
            let b = self.b.emit(Inst::Advice(Hint::Bit(v, k)));
            self.b.emit(Inst::AssertBool(b));
            bits.push(b);
        }
        bits
    }

    /** `Σ 2^k bits[k]`, by doubling from the top bit down; `None` for no bits. */
    pub(super) fn horner(&mut self, bits: &[V]) -> Option<V> {
        let mut top = bits.iter().rev();
        let mut acc = *top.next()?;
        for &b in top {
            let twice = self.b.add(acc, acc);
            acc = self.b.add(twice, b);
        }
        Some(acc)
    }
}
