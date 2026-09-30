/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The 64 bits of a `field` element's canonical representative (section 7.9). The two
 * 32-bit halves `lo` and `hi` must make up the element, `lo + 2^32 hi = v`, and be
 * canonical: `hi = 2^32 - 1` forces `lo = 0`. The sum is below `2^64`; its only other
 * candidate, `v + p` for `v < 2^32 - 1`, has `hi = 2^32 - 1` and `lo = v + 1`, which the
 * second constraint excludes, so the bits are unique.
 */

use alloc::vec::Vec;

use super::rewrite::Rebuild;
use crate::compiler::ssa::{Inst, V};

impl Rebuild {
    /** The 64 bits of `v`, least significant first. */
    pub(super) fn field_bits(&mut self, v: V) -> Vec<V> {
        if let Some(bits) = self.bits.get(&(v, 64)) {
            return bits.clone();
        }
        let bits = self.advice_bits(v, 0..64);
        let (low, high) = bits.split_at(32);
        let (Some(lo), Some(hi)) = (self.horner(low), self.horner(high)) else {
            return bits;
        };
        let shift = self.b.konst(1 << 32);
        let scaled = self.b.mul(hi, shift);
        let sum = self.b.add(lo, scaled);
        let diff = self.b.sub(sum, v);
        self.b.emit(Inst::AssertZero(diff));
        let top = self.b.konst((1 << 32) - 1);
        let at_top = self.b.emit(Inst::Eq(hi, top));
        let both = self.b.mul(at_top, lo);
        self.b.emit(Inst::AssertZero(both));
        self.bits.insert((v, 64), bits.clone());
        bits
    }
}
