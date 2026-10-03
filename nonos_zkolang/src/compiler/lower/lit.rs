/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Literals as slots: an integer of at most 32 bits is `x mod p`, a 64-bit one the two
 * 32-bit halves of its two's-complement pattern, a `field` element itself.
 */

use alloc::vec::Vec;

use super::cx::Lower;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::ssa::V;
use crate::compiler::tir::TLit;

impl<'p> Lower<'p> {
    /** The slots of the literal `l` of type `t`. */
    pub(super) fn lit(&mut self, l: TLit, t: TyId) -> Vec<V> {
        match l {
            TLit::Unit => Vec::new(),
            TLit::Bool(b) => alloc::vec![self.b.konst(i128::from(b))],
            TLit::Int(v) => self.int_slots(v, t),
        }
    }

    /** The slots of the integer or `field` value `v` of type `t`. */
    pub(super) fn int_slots(&mut self, v: i128, t: TyId) -> Vec<V> {
        match self.p.types.kind(t) {
            TyKind::Int(i) if i.bits() > 32 => {
                let pattern = v.rem_euclid(1i128 << 64);
                let (lo, hi) = (pattern & 0xFFFF_FFFF, pattern >> 32);
                alloc::vec![self.b.konst(lo), self.b.konst(hi)]
            }
            _ => alloc::vec![self.b.konst(v)],
        }
    }
}
