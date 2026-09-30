/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A struct or variant built or matched in the form it is declared in: named, tuple or
 * unit. An enum is built only through one of its variants.
 */

use alloc::format;

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{Form, TyId};
use crate::compiler::syntax::ast::Path;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `Some` if the shape `s`, which `p` names, is built in the form `form`; else reported. */
    pub(super) fn shape_form(&mut self, p: &'a Path, s: Shape, form: Form) -> Option<()> {
        let has = self.sema.types.adt(s.0)?.variants.get(s.1 as usize)?.form;
        if has == form {
            return Some(());
        }
        let name = self.shape_name(s);
        let how = match has {
            Form::Named => format!("`{name} {{ .. }}`"),
            Form::Tuple => format!("`{name}(..)`"),
            Form::Unit => format!("`{name}`"),
        };
        let what = format!("`{name}` is not built this way; build it as {how}");
        let d = Diagnostic::error(Code::WRONG_KIND, what, p.span, "another form");
        self.sema.diags.push(d);
        None
    }

    /** Report `p`, which names the enum `ty` where one of its variants is wanted. */
    pub(super) fn whole_enum<T>(&mut self, p: &'a Path, ty: TyId) -> Option<T> {
        let name = self.show(ty);
        let what = format!("`{name}` is an enum; name one of its variants, as `{name}::V`");
        let d = Diagnostic::error(Code::WRONG_KIND, what, p.span, "an enum, not a variant");
        self.sema.diags.push(d);
        None
    }
}
