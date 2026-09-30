/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Text the lexer found unterminated, which may have run over a closing brace. */

use super::super::parser::Parser;
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Whether a string or block comment after `open` was left unterminated: it may have
     * run over the `}`, and its own error explains this one.
     */
    pub(super) fn left_open_since(&self, open: Span) -> bool {
        let before = self.tokens.get(..self.pos).unwrap_or(&[]);
        let string = before
            .iter()
            .rev()
            .take_while(|t| t.span.lo > open.lo)
            .filter(|t| t.kind == TokenKind::Error)
            .map(|t| self.text_of(*t))
            .any(|s| s.starts_with('"') && (s.len() == 1 || !s.ends_with('"')));
        let comment = self
            .comments
            .iter()
            .rev()
            .take_while(|c| c.span.lo > open.lo)
            .any(|c| {
                let s = self
                    .text
                    .get(c.span.lo as usize..c.span.hi as usize)
                    .unwrap_or("");
                s.starts_with("/*") && !s.ends_with("*/")
            });
        string || comment
    }
}
