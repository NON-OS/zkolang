/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Text outside ASCII where only ASCII is allowed. */

use alloc::format;

use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::{FileId, Span};

/**
 * Report the run of non-ASCII characters at `start`, which is outside any comment or
 * string literal, and return the offset after it.
 */
pub(super) fn non_ascii(
    text: &str,
    start: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> usize {
    let b = text.as_bytes();
    let mut i = start;
    /* A run of non-ASCII characters is one error, not one per byte. */
    while i < len && !b[i].is_ascii() {
        i += 1;
    }
    while !text.is_char_boundary(i) && i < b.len() {
        i += 1;
    }
    let shown = text.get(start..i).unwrap_or("");
    diags.push(
        Diagnostic::error(
            Code::UNEXPECTED_CHAR,
            format!("unexpected character `{shown}`"),
            Span::new(file, start as u32, i as u32),
            "not allowed here",
        )
        .with_help("outside comments and string literals only ASCII is allowed"),
    );
    i
}
