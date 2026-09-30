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
use crate::compiler::ssa::{Inst, V};

/**
 * How far apart in the old program two uses of the bits of one value may be and still
 * share a decomposition. Its bits are held in registers from one to the other; further
 * apart, the value is decomposed again, which costs rows but no registers.
 */
pub(super) const NEAR: usize = 64;

impl Rebuild {
    /** The bits of `v`, least significant first, which must make up `v < 2^n`. */
    pub(super) fn decompose(&mut self, v: V, n: u8) -> Vec<V> {
        match self.bits.get(&(v, n)) {
            Some((bits, at)) if self.at.saturating_sub(*at) <= NEAR => return bits.clone(),
            _ => {}
        }
        let bits = self.fresh(v, n);
        self.bits.insert((v, n), (bits.clone(), self.at));
        bits
    }

    /** Check `v < 2^n`, with bits of its own unless nearby ones already check it. */
    pub(super) fn range_check(&mut self, v: V, n: u8) {
        match self.bits.get(&(v, n)) {
            Some((_, at)) if self.at.saturating_sub(*at) <= NEAR => {}
            _ => {
                self.fresh(v, n);
            }
        }
    }

    /** A new decomposition of `v` into `n` bits. */
    fn fresh(&mut self, v: V, n: u8) -> Vec<V> {
        if n >= 64 {
            /* Outside the gadget's range: the semantics fails, and so does this. */
            let one = self.b.konst(1);
            self.b.emit(Inst::AssertZero(one));
            return Vec::new();
        }
        let (bits, sum) = self.read_bits(v, 0..n);
        let diff = match sum {
            Some(sum) => self.b.sub(sum, v),
            None => v,
        };
        self.b.emit(Inst::AssertZero(diff));
        bits
    }
}
