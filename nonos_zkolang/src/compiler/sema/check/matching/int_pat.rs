/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The integer literals of literal and range patterns, typed by the value they take. */

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::{Lit, PatKind, Pattern};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The value of the integer literal pattern `p`, for a value of type `ty`. */
    pub(super) fn int_end(&mut self, p: &Pattern, ty: TyId) -> Option<i128> {
        let PatKind::Lit {
            lit: Lit::Int { value, suffix, .. },
            negative,
        } = &p.kind
        else {
            let what = "an integer literal stands here";
            let d = Diagnostic::error(Code::PATTERN_MISMATCH, what, p.span, "not an integer");
            self.sema.diags.push(d);
            return None;
        };
        let t = self.int_lit_ty(*suffix, Some(ty));
        if !self.unify(t, ty) {
            self.mismatch(p.span, ty, t);
            return None;
        }
        let v = i128::from(*value);
        Some(if *negative { -v } else { v })
    }
}
