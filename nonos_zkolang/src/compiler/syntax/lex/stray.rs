/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Characters that begin no token. A run of them is reported once and left out of the
 * token stream, as rustc does, so `let x = $y;` reads as `let x = y;` and the one mistake
 * gives one error.
 */

use super::describe::looks_like;
use super::punct::scan_punct;
use super::stray_report::stray_error;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::token::TokenKind;

/**
 * Report the run of characters at `start` that begin no token. Returns the offset after
 * it and, for a lone character that only looks like ASCII punctuation, that punctuation,
 * which parsing goes on with.
 */
pub(super) fn stray(
    text: &str,
    start: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> (Option<TokenKind>, usize) {
    let mut chars = text.get(start..len).unwrap_or("").char_indices();
    let first = chars.next().map_or(' ', |(_, c)| c);
    let mut end = start + first.len_utf8();
    let mut count = 1usize;
    for (i, c) in chars {
        if !begins_nothing(text.as_bytes(), start + i, len, c) {
            break;
        }
        end = start + i + c.len_utf8();
        count += 1;
    }
    let span = Span::new(file, start as u32, end as u32);
    diags.push(stray_error(first, count, span));
    let Some(like) = looks_like(first).filter(|&a| count == 1 && a != ' ') else {
        return (None, end);
    };
    /* Read the look-alike as its ASCII character, joined with what follows: `−=` is `-=`. */
    let mut buf = [like as u8, 0, 0];
    let tail = text.as_bytes().get(end..len).unwrap_or(&[]);
    let n = 1 + tail.iter().take(2).take_while(|c| c.is_ascii()).count();
    buf[1..n].copy_from_slice(&tail[..n - 1]);
    match scan_punct(&buf, 0, n) {
        Some((k, used)) => (Some(k), end + used - 1),
        None => (None, end),
    }
}

/** Whether `c`, at byte `i`, begins no token and is not whitespace. */
fn begins_nothing(b: &[u8], i: usize, len: usize, c: char) -> bool {
    if !c.is_ascii() {
        return true;
    }
    let ok = c.is_ascii_whitespace() || c.is_ascii_alphanumeric() || matches!(c, '_' | '"' | '\'');
    !ok && scan_punct(b, i, len).is_none()
}
