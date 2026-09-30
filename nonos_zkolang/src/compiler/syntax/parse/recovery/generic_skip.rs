/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Recovery inside a generic list. Its `<` is not a bracket the other skips can track, as a
 * `<` may also compare, so the skip looks for the list's `>` on the line where it is.
 */

use super::super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * After an error inside a generic list, skip to the `>` that closes it, leaving it
     * current, and say whether there is one: on the same line, outside brackets, before
     * any brace. Nothing is skipped when there is none.
     */
    pub(in crate::compiler::syntax::parse) fn skip_to_generic_close(&mut self) -> bool {
        if self.split.is_some() {
            return false;
        }
        let (mut brackets, mut angles) = (0usize, 0usize);
        let mut end = None;
        for (i, t) in self.tokens.iter().enumerate().skip(self.pos) {
            if i > self.pos && self.line_end_before(t.span).is_some() {
                break;
            }
            let closes = match t.kind {
                TokenKind::Gt | TokenKind::Ge => 1,
                TokenKind::Shr | TokenKind::ShrEq => 2,
                _ => 0,
            };
            match t.kind {
                TokenKind::Eof | TokenKind::LBrace | TokenKind::RBrace => break,
                TokenKind::LParen | TokenKind::LBracket => brackets += 1,
                TokenKind::RParen | TokenKind::RBracket if brackets == 0 => break,
                TokenKind::RParen | TokenKind::RBracket => brackets -= 1,
                TokenKind::Lt if brackets == 0 => angles += 1,
                _ if brackets == 0 && closes > angles => {
                    end = Some(i);
                    break;
                }
                _ if brackets == 0 => angles -= closes,
                _ => {}
            }
        }
        let Some(end) = end else {
            return false;
        };
        while self.pos < end {
            self.bump();
        }
        true
    }
}
