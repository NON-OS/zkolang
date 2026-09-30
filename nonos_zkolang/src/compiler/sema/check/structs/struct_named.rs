/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The struct a path names. */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefKind;
use crate::compiler::sema::ty::{Form, TyId, Types};
use crate::compiler::syntax::ast::{Path, PathRoot};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The type of the struct `p` names, which must be built in the form `form`; `None`
     * once the path or the form is reported. `Self` for an enum names no variant.
     */
    pub(super) fn struct_named(&mut self, p: &'a Path, form: Form) -> Option<TyId> {
        let ty = match p.root == PathRoot::SelfType && p.segments.is_empty() {
            true => Some(self.sema.self_type(p.span).0).filter(|&t| t != Types::ERROR)?,
            false => self.struct_def(p)?,
        };
        if self.sema.types.adt(ty).is_some_and(|a| a.is_enum) {
            let name = self.show(ty);
            return self.whole_enum(p, &name);
        }
        self.shape_form(p, (ty, 0), form).map(|_| ty)
    }

    /** The type of the struct the path `p` names; `None` once reported. */
    fn struct_def(&mut self, p: &'a Path) -> Option<TyId> {
        let (last, prefix) = p.segments.split_last()?;
        if prefix.iter().any(|s| s.generics.is_some()) {
            self.sema.no_generics(p);
        }
        let names: Vec<&str> = p.segments.iter().map(|s| s.ident.name.as_str()).collect();
        let def = match self.sema.defs.resolve(self.module, p.root, &names) {
            Ok(d) => d,
            Err(e) => {
                self.sema.report_path(p, e);
                return None;
            }
        };
        self.sema.note_use(def, p);
        let kind = self.sema.defs.get(def).map(|d| d.kind);
        if kind == Some(DefKind::Enum) {
            let name = self
                .sema
                .defs
                .get(def)
                .map(|d| d.name.clone())
                .unwrap_or_default();
            return self.whole_enum(p, &name);
        }
        if kind != Some(DefKind::Struct) {
            let d = Diagnostic::error(
                Code::WRONG_KIND,
                format!("`{}` is not a struct", p.last_name()),
                p.span,
                "not a struct",
            );
            self.sema.diags.push(d);
            return None;
        }
        Some(self.instance(def, last.generics.as_deref(), p.span))
    }
}
