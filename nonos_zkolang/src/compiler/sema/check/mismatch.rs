/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reporting a type that does not fit (E0300), and how messages name types. */

use alloc::string::String;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report that a value of type `found` stands at `at`, where `want` is expected. */
    pub(crate) fn mismatch(&mut self, at: Span, want: TyId, found: TyId) {
        let label = alloc::format!(
            "expected `{}`, found `{}`",
            self.show(want),
            self.show(found)
        );
        self.sema.diags.push(Diagnostic::error(
            Code::MISMATCHED_TYPES,
            "mismatched types",
            at,
            label,
        ));
    }

    /** How a message names `t` as it stands now. */
    pub(crate) fn show(&mut self, t: TyId) -> String {
        let t = self.zonk(t, false);
        self.sema.types.display(t)
    }

    /** An expression that failed to check, which the checker has reported. */
    pub(crate) fn error(&self, span: Span) -> TExpr {
        TExpr {
            kind: TExprKind::Error,
            ty: Types::ERROR,
            span,
        }
    }
}
