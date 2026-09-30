/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The 64 bits of a pattern, its halves' bits in order, and back; and a barrel of shifts. */

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

    /**
     * `x << k` when `left`, else `x >> k`, arithmetic if `signed`, for `k < 64`: six
     * stages, each shifting by `2^j` or not as bit `j` of `k` says.
     */
    pub(in crate::compiler::lower) fn shift64(
        &mut self,
        left: bool,
        x: (V, V),
        k: V,
        signed: bool,
    ) -> (V, V) {
        let last = self.b.konst(63);
        let room = self.b.sub(last, k);
        self.require_below(room, 6);
        let kg = self.guarded(k, 0);
        let mut bits = self.bits64(x);
        let zero = self.b.konst(0);
        let fill = match (left, signed) {
            (false, true) => bits.get(63).copied().unwrap_or(zero),
            _ => zero,
        };
        for j in 0..6u8 {
            let on = self.b.emit(Inst::Bit(kg, j, 6));
            let s = 1usize << j;
            let next: Vec<V> = (0..64usize)
                .map(|i| {
                    let from = match left {
                        true => i
                            .checked_sub(s)
                            .and_then(|i| bits.get(i).copied())
                            .unwrap_or(zero),
                        false => bits.get(i + s).copied().unwrap_or(fill),
                    };
                    self.b.sel(on, from, bits[i])
                })
                .collect();
            bits = next;
        }
        self.join_bits64(&bits)
    }
}
