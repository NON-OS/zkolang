/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The struct a path names, and its fields. */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefKind;
use crate::compiler::sema::ty::{Form, TyId, Types};
use crate::compiler::syntax::ast::{Path, PathRoot};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The type of the struct `p` names, which must be built in the form `form`; `None`
     * once the path or the form is reported.
     */
    pub(super) fn struct_named(&mut self, p: &'a Path, form: Form) -> Option<TyId> {
        let ty = match p.root == PathRoot::SelfType && p.segments.is_empty() {
            true => Some(self.sema.self_type(p.span).0).filter(|&t| t != Types::ERROR)?,
            false => self.struct_def(p)?,
        };
        self.struct_form(p, ty, form)
    }

    /** The type of the struct the path `p` names; `None` once reported. */
    fn struct_def(&mut self, p: &'a Path) -> Option<TyId> {
        self.sema.no_generics(p);
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let def = match self.sema.defs.resolve(self.module, p.root, &names) {
            Ok(d) => d,
            Err(e) => {
                self.sema.report_path(p, e);
                return None;
            }
        };
        self.sema.note_use(def, p);
        if self.sema.defs.get(def).map(|d| d.kind) != Some(DefKind::Struct) {
            let d = Diagnostic::error(
                Code::WRONG_KIND,
                format!("`{}` is not a struct", p.last_name()),
                p.span,
                "not a struct",
            );
            self.sema.diags.push(d);
            return None;
        }
        Some(self.sema.struct_ty(def).0)
    }

    /** The name and type of each field of the struct `ty`. */
    pub(super) fn struct_fields(&self, ty: TyId) -> Vec<(Option<String>, TyId)> {
        let v = self.sema.types.adt(ty).and_then(|a| a.variants.first());
        v.map(|v| v.fields.iter().map(|f| (f.name.clone(), f.ty)).collect())
            .unwrap_or_default()
    }
}
