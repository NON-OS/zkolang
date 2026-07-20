/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Scan an operator or a piece of punctuation.

use super::token::Tok;
use crate::lang::CompileError;

/// Read a single or multi-character symbol at `i`. `==`, `!=`, and `..` are the
/// two-character forms; a lone `!` or `.` is a lexing error. `None` means the byte
/// begins no symbol, which the caller reports as an unexpected character.
pub(super) fn scan_symbol(b: &[u8], i: usize) -> Result<Option<(Tok, usize)>, CompileError> {
    let single = match b[i] {
        b'+' => Some(Tok::Plus),
        b'-' => Some(Tok::Minus),
        b'*' => Some(Tok::Star),
        b'/' => Some(Tok::Slash),
        b'(' => Some(Tok::LParen),
        b')' => Some(Tok::RParen),
        b',' => Some(Tok::Comma),
        b';' => Some(Tok::Semi),
        b'{' => Some(Tok::LBrace),
        b'}' => Some(Tok::RBrace),
        b'[' => Some(Tok::LBracket),
        b']' => Some(Tok::RBracket),
        _ => None,
    };
    if let Some(t) = single {
        return Ok(Some((t, i + 1)));
    }
    match b[i] {
        b'=' if b.get(i + 1) == Some(&b'=') => Ok(Some((Tok::EqEq, i + 2))),
        b'=' => Ok(Some((Tok::Assign, i + 1))),
        b'!' if b.get(i + 1) == Some(&b'=') => Ok(Some((Tok::BangEq, i + 2))),
        b'.' if b.get(i + 1) == Some(&b'.') => Ok(Some((Tok::DotDot, i + 2))),
        b'!' | b'.' => Err(CompileError::UnexpectedChar { at: i }),
        _ => Ok(None),
    }
}
