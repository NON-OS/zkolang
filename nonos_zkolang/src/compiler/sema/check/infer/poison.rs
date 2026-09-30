/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A type met with the error type: the general variables in it stand for the error type
 * too, so an error already reported is not reported again as a type nothing settles.
 */

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind, Types};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Bind every general variable in `a` and `b`, one of them the error type, to it. */
    pub(super) fn poison(&mut self, a: TyId, b: TyId) -> bool {
        self.poison_in(a);
        self.poison_in(b);
        true
    }

    fn poison_in(&mut self, t: TyId) {
        match self.kind(t) {
            TyKind::Infer(x) => self.vars.bind(x, Types::ERROR),
            TyKind::Tuple(ts) => ts.iter().for_each(|&e| self.poison_in(e)),
            TyKind::Array(e, _) => self.poison_in(e),
            TyKind::Adt(_) => self.type_args(t).iter().for_each(|&e| self.poison_in(e)),
            _ => {}
        }
    }
}
