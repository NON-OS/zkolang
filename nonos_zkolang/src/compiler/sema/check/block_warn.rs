/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What a block warns of: unreachable code (W0003), and a value computed and dropped (W0002). */

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Warn of code after a statement that leaves the block (W0003). */
    pub(super) fn unreachable(&mut self, at: Span) {
        self.sema.diags.push(Diagnostic::warning(
            Code::UNREACHABLE,
            "unreachable code",
            at,
            "no run reaches this",
        ));
    }

    /** Warn of a value computed and dropped (W0002). */
    pub(super) fn discarded(&mut self, at: Span) {
        let d = Diagnostic::warning(
            Code::UNUSED_VALUE,
            "this value is not used",
            at,
            "computed and dropped",
        )
        .with_help("bind it with `let` if it is wanted, or remove it");
        self.sema.diags.push(d);
    }
}

/** Whether evaluating `e` does nothing but compute a value. */
pub(super) fn discarded(e: &TExpr) -> bool {
    matches!(
        e.kind,
        TExprKind::Lit(_)
            | TExprKind::Local(_)
            | TExprKind::Const(_)
            | TExprKind::Unary(..)
            | TExprKind::Chain(..)
            | TExprKind::Cast(_)
            | TExprKind::Tuple(_)
            | TExprKind::Record(_)
            | TExprKind::Array(_)
            | TExprKind::TupleField(..)
    )
}
