/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One token, chosen by its first byte. */

use super::number_token::number;
use super::punct::scan_punct;
use super::quote_char::quote_char;
use super::raw_ident::raw_ident;
use super::stray::stray;
use super::string_scan::scan_string;
use super::word::word;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::token::TokenKind;

/**
 * Scan the token at `start`, which begins neither whitespace nor a comment. Returns its
 * kind and the offset after it. A malformed literal is reported and scanned as an `Error`
 * token, which stands for a value; text that stands for nothing is reported and yields no
 * token.
 */
pub(super) fn scan_token(
    text: &str,
    start: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> (Option<TokenKind>, usize) {
    let b = text.as_bytes();
    let c = b[start];
    let span = |lo: usize, hi: usize| Span::new(file, lo as u32, hi as u32);
    if c.is_ascii_alphanumeric() || c == b'_' {
        let mut i = start;
        while i < len && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
            i += 1;
        }
        if &text[start..i] == "r" && b.get(i) == Some(&b'#') {
            if let Some(end) = raw_ident(text, start, i + 1, len, file, diags) {
                return (Some(TokenKind::Ident), end);
            }
        }
        let kind = if c.is_ascii_digit() {
            number(&text[start..i], span(start, i), diags)
        } else {
            word(text, start, i, span(start, i), diags)
        };
        return (Some(kind), i);
    }
    match c {
        b'"' => {
            let (next, ok) = scan_string(&text[..len], start, file, diags);
            (
                Some(if ok { TokenKind::Str } else { TokenKind::Error }),
                next,
            )
        }
        b'\'' => quote_char(&text[..len], start, file, diags),
        _ => match scan_punct(b, start, len) {
            Some((kind, next)) => (Some(kind), next),
            None => stray(text, start, len, file, diags),
        },
    }
}
