/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Generic `impl` blocks (section 10.2): `impl<T> G<T>` is for a struct or enum declared in
 * this crate. Each function of one is a template, registered under its item and name and
 * instantiated for the arguments a use binds its block's parameters to. A name is given
 * to at most one function of a type, over every `impl` block for it.
 */

use super::cx::Sema;
use super::defs::{DefId, DefKind};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{ImplDecl, ItemKind, TypeKind};

impl<'a> Sema<'a> {
    /** Register the functions of the generic `impl` block `imp`, the item `item` of `m`. */
    pub(super) fn generic_impl(&mut self, imp: &'a ImplDecl, m: DefId) {
        let target = match &imp.self_ty.kind {
            TypeKind::Path(p) => self.type_def(m, p),
            _ => None,
        };
        let kind = target.and_then(|d| self.defs.get(d)).map(|d| d.kind);
        let (Some(adt), Some(DefKind::Struct | DefKind::Enum)) = (target, kind) else {
            let what = "a generic `impl` is for a struct or enum declared in this crate";
            let d = Diagnostic::error(Code::WRONG_KIND, what, imp.self_ty.span, "not one");
            self.diags.push(d);
            return;
        };
        for member in &imp.items {
            if let ItemKind::Fn(f) = &member.kind {
                self.generic_member(adt, (imp, member, f), m);
            }
        }
    }
}
