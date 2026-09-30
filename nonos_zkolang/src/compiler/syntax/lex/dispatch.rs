/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One token, chosen by its first byte. */

use alloc::format;

use super::non_ascii::non_ascii;
use super::number_token::number;
use super::punct::scan_punct;
use super::string::scan_string;
use super::word::word;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::token::TokenKind;

/**
 * Scan the token at `start`, which begins neither whitespace nor a comment. Returns its
 * kind and the offset after it; a problem is reported and scanned as an `Error` token.
 */
pub(super) fn scan_token(
    text: &str,
    start: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> (TokenKind, usize) {
    let b = text.as_bytes();
    let c = b[start];
    let span = |lo: usize, hi: usize| Span::new(file, lo as u32, hi as u32);
    let mut i = start;
    let kind = if c.is_ascii_alphanumeric() || c == b'_' {
        while i < len && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
            i += 1;
        }
        if c.is_ascii_digit() {
            number(&text[start..i], span(start, i), diags)
        } else {
            word(text, start, i, span(start, i), diags)
        }
    } else if c == b'"' {
        let (next, problem) = scan_string(b, i, len);
        i = next;
        match problem {
            None => TokenKind::Str,
            Some((code, msg, at)) => {
                diags.push(Diagnostic::error(code, msg, span(at, at + 1), ""));
                TokenKind::Error
            }
        }
    } else if c.is_ascii() {
        match scan_punct(b, i, len) {
            Some((kind, next)) => {
                i = next;
                kind
            }
            None => {
                i += 1;
                diags.push(Diagnostic::error(
                    Code::UNEXPECTED_CHAR,
                    format!("unexpected character `{}`", c.escape_ascii()),
                    span(start, i),
                    "this character begins no token",
                ));
                TokenKind::Error
            }
        }
    } else {
        i = non_ascii(text, start, len, file, diags);
        TokenKind::Error
    };
    (kind, i)
}
