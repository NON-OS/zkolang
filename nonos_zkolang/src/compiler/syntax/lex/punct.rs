/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Operators and punctuation, longest match first. */

use super::punct_one::one_byte;
use crate::compiler::syntax::token::TokenKind;

/**
 * The punctuation token at `i` and the offset after it, or `None` when the byte begins no
 * token.
 */
pub(super) fn scan_punct(b: &[u8], i: usize, len: usize) -> Option<(TokenKind, usize)> {
    use TokenKind::*;
    let at = |k: usize| {
        if i + k < len {
            b.get(i + k).copied()
        } else {
            None
        }
    };
    let three = match (at(0)?, at(1), at(2)) {
        (b'<', Some(b'<'), Some(b'=')) => Some(ShlEq),
        (b'>', Some(b'>'), Some(b'=')) => Some(ShrEq),
        (b'.', Some(b'.'), Some(b'=')) => Some(DotDotEq),
        _ => None,
    };
    if let Some(k) = three {
        return Some((k, i + 3));
    }
    let two = match (at(0)?, at(1)) {
        (b'&', Some(b'&')) => Some(AmpAmp),
        (b'|', Some(b'|')) => Some(PipePipe),
        (b'<', Some(b'<')) => Some(Shl),
        (b'>', Some(b'>')) => Some(Shr),
        (b'+', Some(b'=')) => Some(PlusEq),
        (b'-', Some(b'=')) => Some(MinusEq),
        (b'*', Some(b'=')) => Some(StarEq),
        (b'/', Some(b'=')) => Some(SlashEq),
        (b'%', Some(b'=')) => Some(PercentEq),
        (b'^', Some(b'=')) => Some(CaretEq),
        (b'&', Some(b'=')) => Some(AmpEq),
        (b'|', Some(b'=')) => Some(PipeEq),
        (b'=', Some(b'=')) => Some(EqEq),
        (b'!', Some(b'=')) => Some(Ne),
        (b'<', Some(b'=')) => Some(Le),
        (b'>', Some(b'=')) => Some(Ge),
        (b'.', Some(b'.')) => Some(DotDot),
        (b':', Some(b':')) => Some(ColonColon),
        (b'-', Some(b'>')) => Some(Arrow),
        (b'=', Some(b'>')) => Some(FatArrow),
        _ => None,
    };
    if let Some(k) = two {
        return Some((k, i + 2));
    }
    let one = one_byte(at(0)?)?;
    Some((one, i + 1))
}
