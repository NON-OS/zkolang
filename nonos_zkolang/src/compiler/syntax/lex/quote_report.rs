/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The diagnostic for text that a single quote begins. */

use super::quote_char::Quoted;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

/** The error for the text at `at` that a single quote began. */
pub(super) fn quote_error(at: Span, what: Quoted) -> Diagnostic {
    let (label, help) = match what {
        Quoted::Value => (
            "a character or string in single quotes",
            "the language has no characters; strings are written with `\"`",
        ),
        Quoted::Lifetime => (
            "a label or lifetime",
            "the language has no labels or lifetimes; `break` ends the innermost loop",
        ),
        Quoted::Lone => (
            "this character begins no token",
            "strings are written with `\"`",
        ),
    };
    Diagnostic::error(Code::UNEXPECTED_CHAR, "unexpected character `'`", at, label).with_help(help)
}
