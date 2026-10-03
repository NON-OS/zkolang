/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading the bits of a value from the top down, summing them as they are read. */

use alloc::vec::Vec;

use super::rewrite::Rebuild;
use crate::compiler::ssa::{Hint, Inst, V};

impl Rebuild {
    /**
     * The advice bits `ks` of `v`, least significant first, each constrained to 0 or 1, and
     * their sum `Σ 2^(k - start) b_k`. The bits are read from the top down, each doubled
     * into the sum as it is read, so only the sum and one bit need a register at a time.
     */
    pub(super) fn read_bits(&mut self, v: V, ks: core::ops::Range<u8>) -> (Vec<V>, Option<V>) {
        let mut bits = Vec::with_capacity(ks.len());
        let mut sum: Option<V> = None;
        for k in ks.rev() {
            let b = self.b.emit(Inst::Advice(Hint::Bit(v, k)));
            self.b.emit(Inst::AssertBool(b));
            sum = Some(match sum {
                Some(acc) => {
                    let twice = self.b.add(acc, acc);
                    self.b.add(twice, b)
                }
                None => b,
            });
            bits.push(b);
        }
        bits.reverse();
        (bits, sum)
    }
}
