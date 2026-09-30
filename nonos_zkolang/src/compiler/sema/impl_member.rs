/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One function of an `impl` block, registered under its type and name. */

use alloc::format;
use alloc::string::String;

use super::cx::{FnInfo, Sema};
use super::defs::{Def, DefId, DefKind};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{FnDecl, Item};
use crate::compiler::tir::FnId;

impl<'a> Sema<'a> {
    /** Register the function `f`, the item `member` of an `impl` block for `ty` in `m`. */
    pub(super) fn member(&mut self, ty: TyId, member: &'a Item, f: &'a FnDecl, m: DefId) {
        let key = (ty, String::from(f.name.name.as_str()));
        let generic = self.types.adt(ty).map(|a| (DefId(a.def), key.1.clone()));
        if self.assoc.contains_key(&key)
            || generic.is_some_and(|g| self.generic_assoc.contains_key(&g))
        {
            let what = format!(
                "`{}` already has a function `{}`",
                self.types.display(ty),
                f.name.name
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
            name: f.name.name.clone(),
            span: f.name.span,
            parent: Some(m),
            vis: member.vis,
            item: Some(member),
        });
        let fid = FnId(u32::try_from(self.fns.len()).unwrap_or(u32::MAX));
        self.fns.push(FnInfo::new(def, f, m, Some(ty)));
        self.fn_of.insert(def, fid);
        self.assoc.insert(key, fid);
    }
}
