/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Making two types one (section 5.7): equal types agree, a literal variable takes an
 * integer or `field` type, and the error type agrees with any, so one mistake is reported
 * once. A value of type `!` fits where any type is expected.
 */

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The type `t` stands for now. */
    pub(crate) fn resolve(&mut self, t: TyId) -> TyId {
        self.vars.resolve(&mut self.sema.types, t)
    }

    /** What `t` is now. */
    pub(crate) fn kind(&mut self, t: TyId) -> TyKind {
        let t = self.resolve(t);
        self.sema.types.kind(t).clone()
    }

    /** Make `a` and `b` one type, binding variables; false if they differ. */
    pub(crate) fn unify(&mut self, a: TyId, b: TyId) -> bool {
        let (a, b) = (self.resolve(a), self.resolve(b));
        if a == b {
            return true;
        }
        match (
            self.sema.types.kind(a).clone(),
            self.sema.types.kind(b).clone(),
        ) {
            (TyKind::Error, _) | (_, TyKind::Error) => true,
            (TyKind::Var(x) | TyKind::Infer(x), TyKind::Var(y) | TyKind::Infer(y)) => {
                self.vars.join(x, y);
                true
            }
            (TyKind::Infer(x), _) => self.bind_general(x, b),
            (_, TyKind::Infer(y)) => self.bind_general(y, a),
            (TyKind::Var(x), TyKind::Int(_) | TyKind::Field) => {
                self.vars.bind(x, b);
                true
            }
            (TyKind::Int(_) | TyKind::Field, TyKind::Var(y)) => {
                self.vars.bind(y, a);
                true
            }
            (TyKind::Tuple(xs), TyKind::Tuple(ys)) if xs.len() == ys.len() => xs
                .iter()
                .zip(ys)
                .fold(true, |ok, (x, y)| self.unify(*x, y) && ok),
            (TyKind::Array(x, n), TyKind::Array(y, m)) if n == m => self.unify(x, y),
            (TyKind::Adt(_), TyKind::Adt(_)) => self.unify_adts(a, b),
            _ => false,
        }
    }

    /** Whether a value of type `found` may stand where `want` is expected. */
    pub(crate) fn fits(&mut self, found: TyId, want: TyId) -> bool {
        self.kind(found) == TyKind::Never || self.unify(found, want)
    }

    /** Check that `e` fits `want`, reporting a mismatch (E0300). */
    pub(crate) fn coerce(&mut self, e: &TExpr, want: TyId) {
        if !self.fits(e.ty, want) {
            self.mismatch(e.span, want, e.ty);
        }
    }
}
