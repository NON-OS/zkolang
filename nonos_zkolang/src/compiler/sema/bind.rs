/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic parameters put in scope, each standing for its argument, and taken out again. */

use alloc::vec::Vec;

use super::cx::Sema;
use crate::compiler::sema::ty::GenArg;
use crate::compiler::syntax::ast::GenericParam;
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /**
     * Put the generic parameters `params` in scope, each standing for its argument, and
     * return what was in scope; `None`, with nothing changed, if the numbers differ.
     */
    pub(crate) fn bind_generics<'p>(
        &mut self,
        params: impl IntoIterator<Item = &'p GenericParam>,
        args: &[GenArg],
    ) -> Option<Vec<(alloc::string::String, GenArg)>> {
        let names: Vec<_> = params.into_iter().map(|p| p.name().name.clone()).collect();
        if names.len() != args.len() {
            return None;
        }
        let bound = names.into_iter().zip(args.iter().copied());
        Some(core::mem::replace(&mut self.generics, bound.collect()))
    }

    /**
     * Put the generic parameters of function `f` in scope, standing for `args`, and return
     * what was in scope; with other arguments than it takes, nothing changes.
     */
    pub(crate) fn bind_fn(
        &mut self,
        f: FnId,
        args: &[GenArg],
    ) -> Vec<(alloc::string::String, GenArg)> {
        let params: Vec<&'a GenericParam> = self
            .fns
            .get(f.0 as usize)
            .map(|i| i.params().collect())
            .unwrap_or_default();
        self.bind_generics(params, args)
            .unwrap_or_else(|| self.generics.clone())
    }
}
