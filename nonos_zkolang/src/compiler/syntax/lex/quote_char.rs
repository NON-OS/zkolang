/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The single quote, which begins no token of the language. After it comes what a Rust
 * programmer meant: a character literal, which stands for a value and so becomes one error
 * token; or a label or lifetime, which is reported and left out of the token stream with
 * the `:` of a label, so `'outer: for ...` reads as `for ...`.
 */

use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
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
    let rest = text.get(start + 1..).unwrap_or("");
    let lit = match rest.as_bytes() {
        [b'\\', _, b'\'', ..] => Some(4),
        _ => rest
            .chars()
            .next()
            .filter(|&c| c != '\n' && c != '\r')
            .and_then(|c| {
                (rest.as_bytes().get(c.len_utf8()) == Some(&b'\'')).then(|| c.len_utf8() + 2)
            }),
    };
    let word = rest
        .bytes()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == b'_')
        .count();
    let (end, token, label, help) = match lit {
        Some(n) => (
            start + n,
            Some(TokenKind::Error),
            "a character literal",
            "the language has no characters; strings are written with `\"`",
        ),
        None if word > 0 => {
            let label_colon =
                b.get(start + 1 + word) == Some(&b':') && b.get(start + 2 + word) != Some(&b':');
            let end = start + 1 + word + usize::from(label_colon);
            (
                end,
                None,
                "a label or lifetime",
                "the language has no labels or lifetimes; `break` ends the innermost loop",
            )
        }
        None => (
            start + 1,
            None,
            "this character begins no token",
            "strings are written with `\"`",
        ),
    };
    let span = Span::new(file, start as u32, end as u32);
    let d = Diagnostic::error(
        Code::UNEXPECTED_CHAR,
        "unexpected character `'`",
        span,
        label,
    )
    .with_help(help);
    diags.push(d);
    (token, end)
}
