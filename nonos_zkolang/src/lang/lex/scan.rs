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

/// Tokenize `src`, or report the first byte that begins no valid token.
pub fn lex(src: &str) -> Result<Vec<Tok>, CompileError> {
    let b = src.as_bytes();
    let mut toks: Vec<Tok> = Vec::new();
    let mut i = 0usize;
    while i < b.len() {
        let ch = b[i];
        if ch.is_ascii_whitespace() {
            i += 1;
        } else if ch == b'/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
        } else if is_ident_start(ch) {
            let (t, ni) = scan_word(src, b, i);
            toks.push(t);
            i = ni;
        } else if ch.is_ascii_digit() {
            let (t, ni) = scan_number(b, i)?;
            toks.push(t);
            i = ni;
        } else if let Some((t, ni)) = scan_symbol(b, i)? {
            toks.push(t);
            i = ni;
        } else {
            return Err(CompileError::UnexpectedChar { at: i });
        }
    }
    Ok(toks)
}
