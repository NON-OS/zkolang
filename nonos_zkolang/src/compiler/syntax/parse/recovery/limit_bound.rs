/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A `limit` written with no bound before the loop's body. */

use super::super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Whether the `{` after `limit` opens the loop's body rather than a bound: its group is
     * not followed by another `{`.
     */
    pub(in crate::compiler::syntax::parse) fn limit_without_bound(&self) -> bool {
        if !self.at(TokenKind::LBrace) {
            return false;
        }
        let mut depth: usize = 0;
        for (i, t) in self.tokens.iter().enumerate().skip(self.pos) {
            match t.kind {
                TokenKind::LBrace => depth += 1,
                TokenKind::RBrace if depth <= 1 => {
                    let next = self.tokens.get(i + 1).map(|t| t.kind);
                    return next != Some(TokenKind::LBrace);
                }
                TokenKind::RBrace => depth -= 1,
                TokenKind::Eof => return false,
                _ => {}
            }
        }
        false
    }
}
