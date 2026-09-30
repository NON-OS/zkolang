/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The end of a body: every free literal variable takes its default, the checks left for
 * this point run, and every type in the body is rewritten to what it stands for.
 */

use super::cx::FnCx;
use super::deferred::Deferred;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyKind, Types};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Give every free variable its default, and run the checks left for the end. */
    pub(crate) fn settle(&mut self) {
        for n in 0..self.vars.len() {
            let t = self.sema.types.intern(TyKind::Var(n));
            self.zonk(t, true);
        }
        for d in core::mem::take(&mut self.deferred) {
            match d {
                Deferred::IntOnly { ty, span, op } => {
                    if self.kind(ty) == TyKind::Field {
                        self.no_operator(op, ty, span);
                    }
                }
                Deferred::Negatable { ty, span } => {
                    if matches!(self.kind(ty), TyKind::Int(i) if !i.signed()) {
                        self.no_operator("-", ty, span);
                    }
                }
                Deferred::RangeInt { ty, span } => {
                    if !matches!(self.kind(ty), TyKind::Int(_) | TyKind::Error) {
                        let shown = self.show(ty);
                        let d = Diagnostic::error(
                            Code::MISMATCHED_TYPES,
                            "a `for` range goes over integers",
                            span,
                            alloc::format!("bounds of type `{shown}`"),
                        );
                        self.sema.diags.push(d);
                    }
                }
            }
        }
        for i in 0..self.locals.len() {
            let t = self.locals.get(i).map_or(Types::ERROR, |l| l.ty);
            let z = self.zonk(t, true);
            if let Some(l) = self.locals.get_mut(i) {
                l.ty = z;
            }
        }
    }
}
