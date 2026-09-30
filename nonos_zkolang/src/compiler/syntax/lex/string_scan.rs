/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Scanning string literals, which appear only as `assert` messages and attribute values.
 * A string ends at its closing quote on the line it starts. When a line ends first and a
 * later line holds a quote that closes a statement or an argument, the string is taken to
 * run to it, so the one mistake is reported once and the text after it lexes as intended.
 */

use super::late_quote::late_quote;
use super::string_report::{bad_escape, late_close, unclosed};
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};

/**
 * Scan the string literal whose opening quote is at `i`, reporting every unknown escape
 * and a missing or late closing quote. Returns the offset after it and whether it is well
 * formed.
 */
pub(super) fn scan_string(
    text: &str,
    i: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> (usize, bool) {
    let b = text.as_bytes();
    let span = |lo: usize, hi: usize| Span::new(file, lo as u32, hi as u32);
    let mut ok = true;
    let mut j = i + 1;
    let mut line_end = None;
    while let Some(&c) = b.get(j) {
        match c {
            b'"' => break,
            b'\n' | b'\r' => {
                line_end = Some(j);
                match late_quote(b, j) {
                    Some(q) => j = q,
                    None => break,
                }
            }
            b'\\' => {
                let e = text.get(j + 1..).and_then(|t| t.chars().next());
                let next = j
                    + 1
                    + e.filter(|&e| e != '\n' && e != '\r')
                        .map_or(0, char::len_utf8);
                if !matches!(e, Some('"' | '\\' | 'n' | 't' | '0')) {
                    ok = false;
                    diags.push(bad_escape(span(j, next.max(j + 1)), e));
                }
                j = next;
            }
            _ => j += 1,
        }
    }
    match (line_end, b.get(j) == Some(&b'"')) {
        (None, true) => (j + 1, ok),
        (Some(_), true) => {
            diags.push(late_close(span(i, i + 1), span(j, j + 1)));
            (j + 1, false)
        }
        (end, false) => {
            diags.push(unclosed(span(i, i + 1)));
            (end.unwrap_or(j.min(b.len())), false)
        }
    }
}
