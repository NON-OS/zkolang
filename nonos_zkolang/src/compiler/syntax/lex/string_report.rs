/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Malformed string literals: where a string that runs past its line is taken to end, and
 * the diagnostics.
 */

use alloc::format;
use alloc::string::String;

use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

/** An escape that is not one of the five, at `at`; `e` is the escaped character. */
pub(super) fn bad_escape(at: Span, e: Option<char>) -> Diagnostic {
    let label = match e {
        Some(c) if c.is_ascii_graphic() => format!("unknown escape `\\{c}`"),
        Some(c) if c != '\n' && c != '\r' => format!("unknown escape of U+{:04X}", c as u32),
        _ => String::from("a backslash at the end of a line"),
    };
    Diagnostic::error(
        Code::BAD_ESCAPE,
        "unknown escape in a string literal",
        at,
        label,
    )
    .with_help("the escapes are `\\\"`, `\\\\`, `\\n`, `\\t` and `\\0`")
}

/** A string opened at `open` whose closing quote, at `close`, is on a later line. */
pub(super) fn late_close(open: Span, close: Span) -> Diagnostic {
    Diagnostic::error(
        Code::UNTERMINATED_STRING,
        "a string literal ends on the line it starts",
        open,
        "this string runs onto a later line",
    )
    .with_label(close, "where it ends")
    .with_help("write `\\n` for a line break inside a string")
}

/** A string opened at `open` that nothing closes. */
pub(super) fn unclosed(open: Span) -> Diagnostic {
    Diagnostic::error(
        Code::UNTERMINATED_STRING,
        "unterminated string literal",
        open,
        "this string is never closed",
    )
    .with_help("close the string with `\"` on the line it starts")
}

/**
 * The offset of the quote on a later line that closes a string whose line ended at `at`:
 * the first quote after it, when what follows that quote ends a statement or argument.
 */
pub(super) fn late_quote(b: &[u8], at: usize) -> Option<usize> {
    let q = at + b.get(at..)?.iter().position(|&c| c == b'"')?;
    let after = b.get(q + 1..)?.iter().find(|&&c| c != b' ' && c != b'\t');
    let ends = matches!(
        after,
        None | Some(b';' | b',' | b')' | b']' | b'}' | b'\n' | b'\r')
    );
    ends.then_some(q)
}
