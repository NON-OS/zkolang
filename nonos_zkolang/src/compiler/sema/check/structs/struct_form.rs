/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A struct built or matched in the form it is declared in: named, tuple or unit. */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{Form, TyId};
use crate::compiler::syntax::ast::Path;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `ty`, if it is built in the form `form`, the struct `p` names; `None` once reported. */
    pub(super) fn struct_form(&mut self, p: &'a Path, ty: TyId, form: Form) -> Option<TyId> {
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
}
