/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The diagnostic for a run of characters that begin no token. */

use alloc::format;

use super::describe::{describe, looks_like};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

/** The error for `count` characters at `span` that begin no token, the first `first`. */
pub(super) fn stray_error(first: char, count: usize, span: Span) -> Diagnostic {
    let d = describe(first);
    let (message, label) = if count == 1 {
        (
            format!("unexpected character {d}"),
            "this character begins no token",
        )
    } else {
        (
            format!("{count} unexpected characters, starting with {d}"),
            "these characters begin no token",
        )
    };
    let e = Diagnostic::error(Code::UNEXPECTED_CHAR, message, span, label);
    match (looks_like(first), first.is_ascii()) {
        (Some(' '), _) => e.with_help(format!("{d} looks like a space; write a plain space")),
        (Some(a), _) => e.with_help(format!("{d} looks like `{a}`; write `{a}`")),
        (None, false) => e.with_help("outside comments and string literals only ASCII is allowed"),
        (None, true) => e,
    }
}
