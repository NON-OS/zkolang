/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One function of a generic `impl` block, registered as a template under its item and name. */

use alloc::format;
use alloc::string::String;

use super::cx::{FnInfo, Sema};
use super::defs::{Def, DefId, DefKind};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{FnDecl, ImplDecl, Item};
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /** Register `f`, the item `member` of the generic `impl` block `imp` for `adt`. */
    pub(super) fn generic_member(
        &mut self,
        adt: DefId,
        (imp, member, f): (&'a ImplDecl, &'a Item, &'a FnDecl),
        m: DefId,
    ) {
        let name = String::from(f.name.name.as_str());
        let taken = self.generic_assoc.contains_key(&(adt, name.clone()))
            || self
                .assoc
                .keys()
                .any(|(t, n)| *n == name && self.types.adt(*t).is_some_and(|a| a.def == adt.0));
        if taken {
            let what = format!(
                "`{}` already has a function `{name}`",
                self.defs.get(adt).map_or("", |d| d.name.as_str())
            );
            self.diags.push(Diagnostic::error(
                Code::DUPLICATE_ITEM,
                what,
                f.name.span,
                "defined again",
            ));
            return;
        }
        let def = self.defs.push(Def {
            kind: DefKind::Fn,
            name: name.clone(),
            span: f.name.span,
            parent: Some(m),
            vis: member.vis,
            item: Some(member),
        });
        let mut info = FnInfo::new(def, f, m, None);
        (info.impl_generics, info.impl_self, info.template) =
            (&imp.generics, Some(&imp.self_ty), true);
        let fid = FnId(u32::try_from(self.fns.len()).unwrap_or(u32::MAX));
        self.fns.push(info);
        self.fn_of.insert(def, fid);
        self.generic_assoc.insert((adt, name), fid);
    }
}
