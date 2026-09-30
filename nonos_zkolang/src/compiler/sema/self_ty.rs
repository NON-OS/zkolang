/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `Self` (section 5.2): inside an `impl` block, the type implemented, with the labels its
 * fields write; anywhere else, an error.
 */

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::tir::Labels;

impl<'a> Sema<'a> {
    /** The type `Self` names at `at`, and its labels. */
    pub(crate) fn self_type(&mut self, at: Span) -> (TyId, Labels) {
        let Some(ty) = self.self_ty else {
            let d = Diagnostic::error(
                Code::WRONG_KIND,
                "`Self` stands only inside an `impl` block",
                at,
                "outside an `impl` block",
            );
            self.diags.push(d);
            return (Types::ERROR, Labels::default());
        };
        (ty, self.labels_of(ty))
    }

    /** The labels the fields of the struct `ty` write, each under its index. */
    pub(crate) fn labels_of(&mut self, ty: TyId) -> Labels {
        match self.types.adt(ty).map(|a| (a.def, a.args.clone())) {
            Some((def, args)) => self.adt_ty(DefId(def), &args).1,
            None => Labels::default(),
        }
    }
}
