/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A variant built or matched in the form it is declared in; `Self` names its arguments already. */

use super::super::cx::FnCx;
use super::shape::Shape;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::DefId;
use crate::compiler::sema::ty::Form;
use crate::compiler::syntax::ast::{Path, PathRoot};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The variant `found`, which `p` names, built in the form `form`; `None` once reported. */
    pub(crate) fn variant_shape(
        &mut self,
        p: &'a Path,
        found: Result<Shape, DefId>,
        form: Form,
    ) -> Option<Shape> {
        let s = match found {
            Ok(s) => s,
            Err(def) => return self.no_variant(p, def),
        };
        let own = p.root == PathRoot::SelfType && p.segments.len() == 1;
        if own && p.segments.last().is_some_and(|s| s.generics.is_some()) {
            let what = "`Self` has its generic arguments already";
            let d = Diagnostic::error(Code::WRONG_GENERICS, what, p.span, "given here");
            self.sema.diags.push(d);
        }
        self.shape_form(p, s, form).map(|_| s)
    }
}
