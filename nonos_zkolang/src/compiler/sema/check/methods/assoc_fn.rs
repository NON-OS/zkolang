/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `T::f` and `Self::f` (section 7.10): a function of an `impl` block, found through its
 * type. It is visible where the `impl` block's module is, or everywhere if `pub`.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefKind;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Path, PathRoot, Visibility};
use crate::compiler::tir::FnId;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The function `p` names through a struct, if `p` names one: `Some(Some(f))` found,
     * `Some(None)` reported; `None` if `p` does not go through a struct.
     */
    pub(crate) fn assoc_fn(&mut self, p: &'a Path) -> Option<Option<FnId>> {
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let (last, prefix) = names.split_last()?;
        let ty = match (p.root, prefix.is_empty()) {
            (PathRoot::SelfType, true) => self.sema.self_type(p.span).0,
            (_, false) => {
                let def = self.sema.defs.resolve(self.module, p.root, prefix).ok()?;
                if self.sema.defs.get(def).map(|d| d.kind) != Some(DefKind::Struct) {
                    return None;
                }
                self.sema.note_use(def, p);
                self.sema.struct_ty(def).0
            }
            _ => return None,
        };
        self.sema.no_generics(p);
        Some(self.member_fn(ty, last, p.span))
    }

    /** The function `name` of the struct `ty`, visible here; `None` once reported. */
    pub(super) fn member_fn(&mut self, ty: TyId, name: &str, at: Span) -> Option<FnId> {
        if ty == Types::ERROR {
            return None;
        }
        let Some(&fid) = self.sema.assoc.get(&(ty, String::from(name))) else {
            let what = format!("`{}` has no function `{name}`", self.show(ty));
            let d = Diagnostic::error(Code::NO_FIELD, what, at, "not found");
            self.sema.diags.push(d);
            return None;
        };
        let info = self.sema.fns.get(fid.0 as usize).map(|f| (f.def, f.module));
        let vis = info.and_then(|(d, _)| self.sema.defs.get(d)).map(|d| d.vis);
        if let (Some((_, m)), Some(Visibility::Private)) = (info, vis) {
            if !self.sema.defs.within(self.module, m) {
                let what = format!("the function `{name}` of `{}` is private", self.show(ty));
                let d = Diagnostic::error(Code::PRIVATE_ITEM, what, at, "private");
                self.sema
                    .diags
                    .push(d.with_help("mark it `pub` in its `impl` block"));
            }
        }
        Some(fid)
    }
}
