/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The scanner: a dispatch loop over the word, number, and symbol readers.

use alloc::vec::Vec;

use super::{
    classify::is_ident_start, number::scan_number, symbol::scan_symbol, token::Tok, word::scan_word,
};
use crate::lang::CompileError;

/** The byte-order mark, skipped where a file starts with one. */
pub(crate) const BOM: char = '\u{feff}';

/// Tokenize `src`, returning each token with the byte offset it starts at, or report
/// the first byte that begins no valid token. The offsets let the parser point a
/// diagnostic at the exact place a token sits in the source.
pub fn lex(src: &str) -> Result<(Vec<Tok>, Vec<usize>), CompileError> {
    let b = src.as_bytes();
    let mut toks: Vec<Tok> = Vec::new();
    let mut spans: Vec<usize> = Vec::new();
    /* A byte-order mark some editors write before the first character is not source. */
    let mut i = if src.starts_with(BOM) {
        BOM.len_utf8()
    } else {
        0
    };
    while i < b.len() {
        let ch = b[i];
        if ch.is_ascii_whitespace() {
            i += 1;
        } else if ch == b'/' && b.get(i + 1) == Some(&b'/') {
            /* A lone carriage return ends a line too, as editors display it. */
            while i < b.len() && b[i] != b'\n' && b[i] != b'\r' {
                i += 1;
            }
        } else if is_ident_start(ch) {
            let (t, ni) = scan_word(src, b, i);
            toks.push(t);
            spans.push(i);
            i = ni;
        } else if ch.is_ascii_digit() {
            let (t, ni) = scan_number(b, i)?;
            toks.push(t);
            spans.push(i);
            i = ni;
        } else if let Some((t, ni)) = scan_symbol(b, i)? {
            toks.push(t);
            spans.push(i);
            i = ni;
        } else {
            return Err(CompileError::UnexpectedChar { at: i });
        }
    }
    Ok((toks, spans))
}
