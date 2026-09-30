/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Function bodies, each checked once, on first need: its parameters bound, its block
 * checked against the result type, then settled and rewritten. A generic function is
 * checked in each instance, its generic parameters standing for the instance's
 * arguments; its template is not checked alone.
 */

use super::cx::{Sema, State};
use crate::compiler::tir::{FnId, TFn};

impl<'a> Sema<'a> {
    /** Check the body of function `f`, if it is not checked yet; say whether it checked. */
    pub(crate) fn check_body(&mut self, f: FnId) -> bool {
        let Some(info) = self.fns.get(f.0 as usize) else {
            return false;
        };
        match info.body {
            _ if info.template => return false,
            State::Done(_) => return true,
            State::Failed | State::Checking => return false,
            State::Unchecked => {}
        }
        let (decl, owner, args, origin) = (info.decl, info.owner, info.args.clone(), info.origin);
        self.set_body(f, State::Checking);
        let generics = self.bind_generics(&decl.generics, &args);
        let generics = generics.unwrap_or_else(|| self.generics.clone());
        let outer = core::mem::replace(&mut self.self_ty, owner);
        let errors = self.diags.error_count();
        let out = self.within(decl.name.span, |s| s.body_of(f));
        self.self_ty = outer;
        self.generics = generics;
        let ok = out.is_some();
        let clean = ok && self.diags.error_count() == errors;
        if let (false, Some(origin)) = (clean, origin) {
            self.instance_failed(f, origin);
        }
        self.set_body(f, out.map_or(State::Failed, State::Done));
        if let Some(info) = self.fns.get_mut(f.0 as usize) {
            info.clean = clean;
        }
        ok
    }

    fn set_body(&mut self, f: FnId, s: State<TFn>) {
        if let Some(info) = self.fns.get_mut(f.0 as usize) {
            info.body = s;
        }
    }
}
