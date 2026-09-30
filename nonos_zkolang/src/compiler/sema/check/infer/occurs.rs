/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The occurs check: a variable bound to a type that contains it would stand for an infinite type. */

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Whether the variable `x` occurs in `t`. */
    pub(super) fn occurs(&mut self, x: u32, t: TyId) -> bool {
        match self.kind(t) {
            TyKind::Var(y) | TyKind::Infer(y) => self.vars.root(y) == self.vars.root(x),
            TyKind::Tuple(ts) => ts.iter().any(|&e| self.occurs(x, e)),
            TyKind::Array(e, _) => self.occurs(x, e),
            TyKind::Adt(_) => self.type_args(t).iter().any(|&e| self.occurs(x, e)),
            _ => false,
        }
    }
}
