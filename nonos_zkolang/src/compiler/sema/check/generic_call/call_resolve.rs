/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A call of a generic function given, once its body settles, the instance for its arguments. */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{GenArg, TyKind};
use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Give the call at `at` of the template `*f` the instance for its settled arguments. */
    pub(crate) fn resolve_call(&mut self, f: &mut FnId, at: Span) {
        if !self.sema.fns.get(f.0 as usize).is_some_and(|i| i.template) {
            return;
        }
        let Some(i) = self.pending.iter().position(|(s, _)| *s == at) else {
            return;
        };
        let (_, args) = self.pending.swap_remove(i);
        let mut settled = Vec::with_capacity(args.len());
        for g in args {
            settled.push(match g {
                GenArg::Type(t) => GenArg::Type(self.zonk(t, true)),
                c => c,
            });
        }
        let unknown =
            |g: &GenArg| matches!(g, GenArg::Type(t) if self.sema.types.kind(*t) == &TyKind::Error);
        if settled.iter().any(unknown) {
            return;
        }
        if let Some(id) = self.sema.instance_fn(*f, &settled, self.fn_id, at) {
            *f = id;
        }
    }
}
