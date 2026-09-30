/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The checks left for the end of a body, run once every literal type is settled. */

use super::cast::castable;
use super::cx::FnCx;
use super::deferred::Deferred;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyKind;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Run the check `d`. */
    pub(super) fn run_deferred(&mut self, d: Deferred) {
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
            Deferred::Cast { ty, to, span } => {
                if !castable(&self.kind(ty), &self.kind(to)) {
                    self.bad_cast(ty, to, span);
                }
            }
        }
    }
}
