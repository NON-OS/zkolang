/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Rust's raw and byte strings, `r"..."`, `r#"..."#`, `b"..."` and `br"..."`, which the
 * language does not have. Each is reported once and scanned whole as one error token, so
 * its `#`s and its text raise nothing more. A raw string may span lines, as in Rust.
 */

use super::foreign_end::foreign_end;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::{FileId, Span};

/**
 * If the word `text[start..word_end]` prefixes a raw or byte string, report it and return
 * the offset after its closing quote and `#`s, or after its line if it has none.
 */
pub(super) fn foreign_string(
    text: &str,
    start: usize,
    word_end: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> Option<usize> {
    let b = text.as_bytes();
    let prefix = text.get(start..word_end)?;
    let raw = matches!(prefix, "r" | "br");
    if !raw && prefix != "b" {
        return None;
    }
    let hashes = b
        .get(word_end..len)?
        .iter()
        .take_while(|c| **c == b'#')
        .count();
    let quote = word_end + hashes;
    if b.get(quote) != Some(&b'"') || (!raw && hashes > 0) {
        return None;
    }
    let end = foreign_end(b, quote, len, raw, hashes)?;
    let end = end.min(len);
    let (what, label) = if raw {
        ("raw strings are not part of the language", "a raw string")
    } else {
        ("byte strings are not part of the language", "a byte string")
    };
    let d = Diagnostic::error(Code::UNEXPECTED_CHAR, what, Span::new(file, start as u32, end as u32), label)
        .with_help("write a string literal, `\"...\"`, which takes `\\\"` and `\\\\` for a quote and a backslash");
    diags.push(d);
    Some(end)
}
