/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The mistakes of a pattern `let` or a parameter binds. */

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::Pattern;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report a pattern `p` that may not match where only one that always does stands. */
    pub(super) fn refutable(&mut self, p: &Pattern) {
        let help = "`let` and parameters take names, `_`, and tuples, arrays or structs of them; test values with `match`";
        let d = Diagnostic::error(
            Code::REFUTABLE_PATTERN,
            "this pattern may not match",
            p.span,
            "a pattern that does not always match",
        );
        self.sema.diags.push(d.with_help(help));
    }

    /** Report a pattern `p` of `n` parts for a value of type `ty` (E0402). */
    pub(super) fn pattern_mismatch(&mut self, p: &Pattern, n: usize, ty: TyId) {
        let shown = self.show(ty);
        let what = alloc::format!("a pattern of {n} parts for a value of type `{shown}`");
        let d = Diagnostic::error(
            Code::PATTERN_MISMATCH,
            what,
            p.span,
            "does not fit the value",
        );
        self.sema.diags.push(d);
    }
}
