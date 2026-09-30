/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Signatures of instances, kept once lowered, and of templates for arguments still to settle. */

use super::cx::{Sema, Sig};
use crate::compiler::sema::ty::GenArg;
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /** The signature of function `f`, which is no template: an instance's for its arguments. */
    pub(crate) fn sig(&mut self, f: FnId) -> Sig {
        let Some(info) = self.fns.get(f.0 as usize) else {
            return self.lower_sig(f);
        };
        if let Some(s) = &info.sig {
            return s.clone();
        }
        let (params, args) = (&info.decl.generics, info.args.clone());
        let outer = self
            .bind_generics(params, &args)
            .unwrap_or_else(|| self.generics.clone());
        let sig = self.lower_sig(f);
        self.generics = outer;
        if let Some(info) = self.fns.get_mut(f.0 as usize) {
            info.sig = Some(sig.clone());
        }
        sig
    }

    /**
     * The signature of the template `f`, its generic parameters standing for `args`, which
     * may hold type variables of the body that calls it; it is not kept.
     */
    pub(crate) fn sig_with(&mut self, f: FnId, args: &[GenArg]) -> Sig {
        let params = self
            .fns
            .get(f.0 as usize)
            .map_or(&[][..], |i| &i.decl.generics);
        let outer = self
            .bind_generics(params, args)
            .unwrap_or_else(|| self.generics.clone());
        let sig = self.lower_sig(f);
        self.generics = outer;
        sig
    }
}
