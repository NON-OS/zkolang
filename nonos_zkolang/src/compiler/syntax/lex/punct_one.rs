/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Operators and punctuation one byte long. */

use crate::compiler::syntax::token::TokenKind;

/** The one-byte punctuation token `c` spells, or `None` when it spells none. */
pub(super) fn one_byte(c: u8) -> Option<TokenKind> {
    use TokenKind::*;
    let one = match c {
        b'+' => Plus,
        b'-' => Minus,
        b'*' => Star,
        b'/' => Slash,
        b'%' => Percent,
        b'^' => Caret,
        b'!' => Bang,
        b'&' => Amp,
        b'|' => Pipe,
        b'=' => Eq,
        b'<' => Lt,
        b'>' => Gt,
        b'.' => Dot,
        b',' => Comma,
        b';' => Semi,
        b':' => Colon,
        b'#' => Pound,
        b'(' => LParen,
        b')' => RParen,
        b'[' => LBracket,
        b']' => RBracket,
        b'{' => LBrace,
        b'}' => RBrace,
        _ => return None,
    };
    Some(one)
}
