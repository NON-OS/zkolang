/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Function bodies, each checked once, on first need: its parameters bound, its block
 * checked against the result type, then settled and rewritten.
 */

use super::check::FnCx;
use super::cx::{Sema, State};
use crate::compiler::tir::{FnId, TFn};

impl<'a> Sema<'a> {
    /** Check the body of function `f`, if it is not checked yet; say whether it checked. */
    pub(crate) fn check_body(&mut self, f: FnId) -> bool {
        let Some(info) = self.fns.get(f.0 as usize) else {
            return false;
        };
        match info.body {
            State::Done(_) => return true,
            State::Failed | State::Checking => return false,
            State::Unchecked => {}
        }
        let (m, decl, def) = (info.module, info.decl, info.def);
        self.set_body(f, State::Checking);
        let sig = self.sig(f);
        let errors = self.diags.error_count();
        let out = self.within(decl.name.span, |s| {
            let mut cx = FnCx::new(s, m, Some(sig.ret));
            let params = cx.params(&decl.params, &sig);
            let (mut body, ty) = cx.block(&decl.body, Some(sig.ret));
            if !cx.fits(ty, sig.ret) {
                let at = body.tail.as_ref().map_or(decl.body.span, |t| t.span);
                cx.mismatch(at, sig.ret, ty);
            }
            cx.settle();
            cx.rewrite_block(&mut body);
            body.each_expr(&mut |e| cx.post(e));
            let locals = core::mem::take(&mut cx.locals);
            Some(TFn {
                def,
                name: decl.name.name.clone(),
                is_const: decl.is_const,
                params,
                ret: sig.ret,
                ret_labels: sig.ret_labels.clone(),
                body,
                locals,
                span: decl.name.span,
            })
        });
        let ok = out.is_some();
        let clean = ok && self.diags.error_count() == errors;
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
