/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The end of a body: a literal type still open where it is cast to an integer type takes
 * that type, every other free literal variable takes its default, the checks left for this
 * point run, and every type in the body is rewritten to what it stands for.
 */

use super::cx::FnCx;
use super::deferred::Deferred;
use crate::compiler::sema::ty::{TyKind, Types};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Give every free variable its type, and run the checks left for the end. */
    pub(crate) fn settle(&mut self) {
        let deferred = core::mem::take(&mut self.deferred);
        deferred.iter().for_each(|d| self.settle_cast(d));
        for n in 0..self.vars.len() {
            let t = self.sema.types.intern(TyKind::Var(n));
            self.zonk(t, true);
        }
        for d in deferred {
            self.run_deferred(d);
        }
        for i in 0..self.locals.len() {
            let t = self.locals.get(i).map_or(Types::ERROR, |l| l.ty);
            let z = self.zonk(t, true);
            if let Some(l) = self.locals.get_mut(i) {
                l.ty = z;
            }
        }
    }

    /** A cast of a literal whose type is still open, to an integer type, gives it that type. */
    pub(super) fn settle_cast(&mut self, d: &Deferred) {
        if let Deferred::Cast { ty, to, .. } = *d {
            let open = matches!(self.kind(ty), TyKind::Var(_));
            if open && matches!(self.kind(to), TyKind::Int(_)) {
                self.unify(ty, to);
            }
        }
    }
}
