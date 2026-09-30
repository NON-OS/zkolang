/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A lone name in a pattern that names a unit variant of the prelude, as `None` does
 * (section 9.1): the variant's pattern, not a binding.
 */

use alloc::format;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{Ident, Pattern};
use crate::compiler::tir::{Labels, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The pattern `p`, the lone name `name`, mutable or not, with the labels `l` of its place. */
    pub(crate) fn lone_name_pat(
        &mut self,
        p: &'a Pattern,
        (name, mutable): (&'a Ident, bool),
        ty: TyId,
        l: Labels,
    ) -> TPat {
        if !mutable {
            if let Some(t) = self.prelude_unit_pat(p, name, ty) {
                return t;
            }
        }
        TPat::Bind(self.pat_bind(name, ty, mutable, l))
    }

    /** The pattern `p`, the lone name `name`, if it names a unit variant of the prelude. */
    fn prelude_unit_pat(&mut self, p: &'a Pattern, name: &Ident, ty: TyId) -> Option<TPat> {
        let ((vty, tag), unit) = self.prelude_variant(&name.name, None, p.span)?;
        if !unit {
            return None;
        }
        if !self.unify(vty, ty) {
            let what = format!(
                "a pattern of `{}` for a value of type `{}`",
                self.show(vty),
                self.show(ty)
            );
            let d = Diagnostic::error(
                Code::PATTERN_MISMATCH,
                what,
                p.span,
                "does not fit the value",
            );
            self.sema.diags.push(d);
            return Some(TPat::Wild);
        }
        if !self.pats.arm {
            self.refutable(p);
        }
        Some(TPat::Variant(tag, Vec::new()))
    }
}
