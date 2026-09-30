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
use crate::compiler::sema::ty::{Form, TyId};
use crate::compiler::syntax::ast::Path;

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The type of the struct `p` names, which must be built in the form `form`; `None`
     * once the path or the form is reported.
     */
    pub(super) fn struct_named(&mut self, p: &'a Path, form: Form) -> Option<TyId> {
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
        let ty = self.sema.struct_ty(def).0;
        let has = self.sema.types.adt(ty)?.variants.first()?.form;
        if has != form {
            let how = match has {
                Form::Named => format!("`{} {{ .. }}`", p.last_name()),
                Form::Tuple => format!("`{}(..)`", p.last_name()),
                Form::Unit => format!("`{}`", p.last_name()),
            };
            let what = format!(
                "`{}` is not built this way; build it as {how}",
                p.last_name()
            );
            let d = Diagnostic::error(Code::WRONG_KIND, what, p.span, "another form of struct");
            self.sema.diags.push(d);
            return None;
        }
        Some(ty)
    }

    /** The name and type of each field of the struct `ty`. */
    pub(super) fn struct_fields(&self, ty: TyId) -> Vec<(Option<String>, TyId)> {
        let v = self.sema.types.adt(ty).and_then(|a| a.variants.first());
        v.map(|v| v.fields.iter().map(|f| (f.name.clone(), f.ty)).collect())
            .unwrap_or_default()
    }
}
