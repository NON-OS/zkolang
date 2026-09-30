/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The diagnostics of a build that checks but does not lower. */

use alloc::format;

use super::build_diag::{one, too_large};
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::lower::LowerError;
use crate::compiler::source::Span;

/** The diagnostic of a lowering failure, at `at` when it has no place of its own. */
pub(super) fn lowering(e: LowerError, at: Span) -> Diagnostics {
    one(match e {
        LowerError::NoMain => Diagnostic::error(
            Code::NO_MAIN,
            "running a program needs `fn main` in the root module",
            at,
            "no `main` here",
        ),
        LowerError::Unsupported(what, span) => Diagnostic::error(
            Code::UNSUPPORTED,
            format!("this build does not compile {what} yet"),
            span,
            "not compiled yet",
        ),
        LowerError::TooLarge => too_large(
            "the program unrolls to more instructions than a provable trace holds",
            at,
        ),
    })
}
