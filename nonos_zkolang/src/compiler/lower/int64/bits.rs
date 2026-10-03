/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The 64 bits of a pattern, its halves' bits in order, and back. */

use alloc::vec::Vec;

use super::super::cx::Lower;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** The 64 bits of the pattern `x`, least significant first. */
    pub(in crate::compiler::lower) fn bits64(&mut self, x: (V, V)) -> Vec<V> {
        let mut out = Vec::with_capacity(64);
        for h in [x.0, x.1] {
            let hg = self.guarded(h, 0);
            out.extend((0..32).map(|k| self.b.emit(Inst::Bit(hg, k, 32))));
        }
        out
    }

    /** The pattern with the bits `bits`, least significant first. */
    pub(in crate::compiler::lower) fn join_bits64(&mut self, bits: &[V]) -> (V, V) {
        let (low, high) = bits.split_at(bits.len().min(32));
        (self.bits_value(low), self.bits_value(high))
    }
}
