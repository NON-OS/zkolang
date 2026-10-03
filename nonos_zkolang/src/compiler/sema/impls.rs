/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `impl` blocks (section 10.2): each function of one is registered under the struct it is
 * for and its name, so `T::f(..)` and `value.m(..)` find it. A block is for a struct
 * declared in this crate, and gives each name to at most one of its type's functions.
 */

use alloc::format;

use super::attr_query::cfg_test;
use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{ImplDecl, Item, ItemKind, PathRoot};

impl<'a> Sema<'a> {
    /** Register the functions of every `impl` block of `items`, in module `m`, and inside. */
    pub(super) fn impls(&mut self, items: &'a [Item], m: DefId) {
        for item in items {
            if !self.defs.testing_in(m) && cfg_test(&item.attrs) {
                continue;
            }
            match &item.kind {
                ItemKind::Impl(imp) => self.impl_block(imp, m),
                ItemKind::Mod(md) => {
                    let inner = self
                        .defs
                        .resolve(m, PathRoot::Plain, &[md.name.name.as_str()]);
                    if let (Some(body), Ok(inner)) = (&md.body, inner) {
                        self.impls(body, inner);
                    }
                }
                _ => {}
            }
        }
    }

    fn impl_block(&mut self, imp: &'a ImplDecl, m: DefId) {
        if !imp.generics.is_empty() {
            return self.generic_impl(imp, m);
        }
        let (ty, _) = self.lower_ty(m, &imp.self_ty);
        if self.types.adt(ty).is_none() {
            if ty != crate::compiler::sema::ty::Types::ERROR {
                let what = format!(
                    "`impl` is for a struct declared in this crate, not `{}`",
                    self.types.display(ty)
                );
                self.diags.push(Diagnostic::error(
                    Code::WRONG_KIND,
                    what,
                    imp.self_ty.span,
                    "not a struct",
                ));
            }
            return;
        }
        for member in &imp.items {
            let ItemKind::Fn(f) = &member.kind else {
                continue;
            };
            self.member(ty, member, f, m);
        }
    }
}
