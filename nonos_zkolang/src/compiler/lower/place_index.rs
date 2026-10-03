/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Checking a run-time index, and the types of the current call's locals. */

use super::cx::Lower;
use crate::compiler::ssa::V;

impl<'p> Lower<'p> {
    /** Fail, where the point runs, unless the index `i` is below `n`. */
    pub(super) fn check_index(&mut self, i: V, n: usize) {
        if n == 0 {
            let zero = self.b.konst(0);
            self.require(zero);
            return;
        }
        let last = self.b.konst((n - 1) as i128);
        let room = self.b.sub(last, i);
        let bits = usize::BITS - (n - 1).leading_zeros();
        self.require_below(room, bits);
    }

    /** The type of local `l` of the current call. */
    pub(super) fn local_ty(
        &self,
        l: crate::compiler::tir::LocalId,
    ) -> crate::compiler::sema::ty::TyId {
        let f = self.frames.last().map(|f| f.f);
        let body = f.and_then(|f| self.p.fns.get(f.0 as usize));
        body.and_then(|b| b.locals.get(l.0 as usize))
            .map_or(crate::compiler::sema::ty::Types::ERROR, |x| x.ty)
    }
}
