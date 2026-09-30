/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A type with every literal variable in it replaced by what it stands for. Finishing a
 * body also gives each free variable its default (section 5.6) and binds it, so every
 * use of the variable agrees.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::IntTy;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `t` with its variables resolved; a free one takes its default when `default`. */
    pub(crate) fn zonk(&mut self, t: TyId, default: bool) -> TyId {
        let t = self.resolve(t);
        match self.sema.types.kind(t).clone() {
            TyKind::Var(n) if default => {
                let d = if self.vars.usize_default(n) {
                    Types::int(IntTy::Usize)
                } else {
                    Types::FIELD
                };
                self.vars.bind(n, d);
                d
            }
            TyKind::Tuple(ts) => {
                let ts: Vec<TyId> = ts.iter().map(|&e| self.zonk(e, default)).collect();
                self.sema.types.intern(TyKind::Tuple(ts))
            }
            TyKind::Array(e, n) => {
                let e = self.zonk(e, default);
                self.sema.types.intern(TyKind::Array(e, n))
            }
            _ => t,
        }
    }
}
