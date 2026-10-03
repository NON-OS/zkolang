/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Rust's raw identifiers, `r#name`, which the language does not have. */

use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::{FileId, Span};

/**
 * Report the raw identifier whose `r` starts at `start` and whose name starts at `name`,
 * after the `#`, and return the offset after it; `None` if no name follows the `#`.
 */
pub(super) fn raw_ident(
    text: &str,
    start: usize,
    name: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> Option<usize> {
    let b = text.as_bytes();
    let n = b
        .get(name..len)?
        .iter()
        .take_while(|c| c.is_ascii_alphanumeric() || **c == b'_')
        .count();
    if n == 0
        || !b
            .get(name)
            .is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_')
    {
        return None;
    }
    let span = Span::new(file, start as u32, (name + n) as u32);
    let d = Diagnostic::error(
        Code::UNEXPECTED_CHAR,
        "a raw identifier",
        span,
        "the language has no raw identifiers",
    )
    .with_help("choose a name that is not a keyword");
    diags.push(d);
    Some(name + n)
}
