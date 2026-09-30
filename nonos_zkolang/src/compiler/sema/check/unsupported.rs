/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Forms this build does not check yet (E0904), and `&mut` outside an argument. */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report `what`, a form this build does not check, at `at`. */
    pub(crate) fn unsupported(&mut self, what: &str, at: Span) -> TExpr {
        let d = Diagnostic::error(
            Code::UNSUPPORTED,
            format!("this build does not check {what} yet"),
            at,
            "not checked yet",
        );
        self.sema.diags.push(d);
        self.error(at)
    }

    /** Report `&mut place` where no `&mut` parameter takes it. */
    pub(crate) fn misplaced_ref_mut(&mut self, at: Span) -> TExpr {
        let d = Diagnostic::error(Code::MISMATCHED_TYPES, "`&mut` stands only as an argument", at, "not an argument for a `&mut` parameter")
            .with_help("there are no references: `&mut place` passes a place to a `&mut` parameter (section 10.3)");
        self.sema.diags.push(d);
        self.error(at)
    }
}
