/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The single quote, which begins no token of the language. After it comes what a Rust
 * or Python programmer meant: a character or a single-quoted string, which stands for a
 * value and so becomes one error token; or a label or lifetime, `'a` with no closing
 * quote after the word, which is reported and left out of the token stream with the `:`
 * of a label, so `'outer: for ...` reads as `for ...`.
 */

use super::quote_report::quote_error;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::token::TokenKind;

/** Scan from the quote at `start`: the token it stands for, if any, and the offset after it. */
pub(super) fn quote_char(
    text: &str,
    start: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> (Option<TokenKind>, usize) {
    let b = text.as_bytes();
    let rest = b.get(start + 1..).unwrap_or(&[]);
    let word = rest
        .iter()
        .take_while(|c| c.is_ascii_alphanumeric() || **c == b'_')
        .count();
    let line = rest
        .iter()
        .position(|&c| c == b'\n' || c == b'\r')
        .unwrap_or(rest.len());
    let close = rest
        .get(..line)
        .and_then(|l| l.iter().position(|&c| c == b'\''));
    let lifetime = word > 0 && rest.get(word) != Some(&b'\'');
    let (end, token, what) = match close {
        Some(c) if !lifetime && c > 0 => (start + c + 2, Some(TokenKind::Error), Quoted::Value),
        _ if word > 0 => {
            let colon = rest.get(word) == Some(&b':') && rest.get(word + 1) != Some(&b':');
            (
                start + 1 + word + usize::from(colon),
                None,
                Quoted::Lifetime,
            )
        }
        _ => (start + 1, None, Quoted::Lone),
    };
    let span = Span::new(file, start as u32, end as u32);
    diags.push(quote_error(span, what));
    (token, end)
}

/** What a single quote began. */
pub(super) enum Quoted {
    /** A character or a single-quoted string: a value. */
    Value,
    /** A label or lifetime. */
    Lifetime,
    /** Nothing: a quote alone. */
    Lone,
}
