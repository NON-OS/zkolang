/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The check of one function's body, with its generic parameters and `Self` in scope. */

use super::check::FnCx;
use super::cx::Sema;
use crate::compiler::tir::{FnId, TFn};

impl<'a> Sema<'a> {
    /** The checked body of function `f`, whose generic parameters are in scope. */
    pub(super) fn body_of(&mut self, f: FnId) -> Option<TFn> {
        let info = self.fns.get(f.0 as usize)?;
        let (m, decl, def) = (info.module, info.decl, info.def);
        let sig = self.sig(f);
        let name = self.fn_name(f);
        let mut cx = FnCx::new(self, m, Some(sig.ret));
        cx.fn_id = Some(f);
        let params = cx.params(&decl.params, &sig);
        let (mut body, ty) = cx.block(&decl.body, Some(sig.ret));
        if !cx.fits(ty, sig.ret) {
            let at = body.tail.as_ref().map_or(decl.body.span, |t| t.span);
            cx.mismatch(at, sig.ret, ty);
        }
        cx.settle();
        cx.warn_unused();
        cx.rewrite_block(&mut body);
        body.each_expr(&mut |e| cx.post(e));
        let locals = core::mem::take(&mut cx.locals);
        Some(TFn {
            def,
            name,
            is_const: decl.is_const,
            params,
            ret: sig.ret,
            ret_labels: sig.ret_labels.clone(),
            body,
            locals,
            span: decl.name.span,
        })
    }
}
