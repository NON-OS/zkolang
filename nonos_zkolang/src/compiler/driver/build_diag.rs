/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The diagnostics of a build that checks but does not compile in the back end. */

use alloc::format;

use super::backend::BackendError;
use crate::compiler::codegen::CodegenError;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::Span;

/** The diagnostic of a back-end failure, for the program at `at`. */
pub(super) fn backend_failure(e: BackendError, at: Span) -> Diagnostics {
    one(match e {
        BackendError::Codegen(CodegenError::TooManySlots) => Diagnostic::error(
            Code::TOO_MANY_IO,
            "the program needs more inputs and advice than 16-bit indices name",
            at,
            "compiling this program",
        ),
        BackendError::Codegen(CodegenError::Pressure(_)) => Diagnostic::error(
            Code::REGISTER_PRESSURE,
            "the program needs more values at once than the 32 registers hold",
            at,
            "compiling this program",
        )
        .with_help("use each secret input and computed value closer to where it is made"),
        _ => Diagnostic::error(
            Code::INTERNAL,
            format!("the compiler failed an internal check: {e:?}"),
            at,
            "compiling this program",
        ),
    })
}

/** The diagnostic of a program with more rows than a provable trace holds. */
pub(super) fn too_large(what: &str, at: Span) -> Diagnostic {
    Diagnostic::error(
        Code::TOO_MANY_ROWS,
        alloc::string::String::from(what),
        at,
        "too large",
    )
}

/** `d` alone. */
pub(super) fn one(d: Diagnostic) -> Diagnostics {
    let mut out = Diagnostics::new();
    out.push(d);
    out
}
