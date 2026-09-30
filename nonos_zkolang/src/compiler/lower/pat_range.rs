/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Range patterns: whether an integer, given as its slots, lies in `lo..=hi`. */

use super::cx::Lower;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::ssa::V;
use crate::compiler::syntax::IntTy;

impl<'p> Lower<'p> {
    /** 1 if `vals`, an integer of type `t`, is in `lo..=hi`, else 0. */
    pub(super) fn in_range(&mut self, vals: &[V], (lo, hi): (i128, i128), t: TyId) -> V {
        let it = match self.p.types.kind(t) {
            TyKind::Int(i) => *i,
            _ => IntTy::U32,
        };
        let (lo, hi) = (self.int_slots(lo, t), self.int_slots(hi, t));
        let below = self.less_slots(vals, &lo, it);
        let above = self.less_slots(&hi, vals, it);
        let either = self.b.add(below, above);
        self.b.not(either)
    }

    /** `a < b` for integers of type `it`, given as their slots. */
    fn less_slots(&mut self, a: &[V], b: &[V], it: IntTy) -> V {
        let zero = self.b.konst(0);
        let at = |v: &[V], k: usize| v.get(k).copied().unwrap_or(zero);
        match it.bits() > 32 {
            true => self.less64((at(a, 0), at(a, 1)), (at(b, 0), at(b, 1)), it),
            false => self.less(at(a, 0), at(b, 0), it),
        }
    }
}
