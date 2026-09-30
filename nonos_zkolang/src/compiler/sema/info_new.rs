/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A function as registered, and the generic parameters in scope in it. */

use alloc::vec::Vec;

use super::cx::State;
use super::defs::DefId;
use super::info::FnInfo;
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{FnDecl, GenericParam};

impl<'a> FnInfo<'a> {
    /** The function `decl`, the item `def` of `module`, declared for `owner` if given. */
    pub fn new(def: DefId, decl: &'a FnDecl, module: DefId, owner: Option<TyId>) -> FnInfo<'a> {
        FnInfo {
            def,
            decl,
            module,
            owner,
            sig: None,
            body: State::Unchecked,
            clean: false,
            template: !decl.generics.is_empty(),
            args: Vec::new(),
            origin: None,
            impl_generics: &[],
            impl_self: None,
        }
    }

    /** The generic parameters in scope in the function: its `impl` block's, then its own. */
    pub fn params(&self) -> impl Iterator<Item = &'a GenericParam> + 'a {
        let (outer, own) = (self.impl_generics, &self.decl.generics);
        outer.iter().chain(own.iter())
    }
}
