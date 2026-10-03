/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Text the lexer found unterminated, which may have run over a closing brace or the rest
 * of the file. Its own error explains what is missing after it, which is not reported
 * again.
 */

use super::super::parser::Parser;
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Whether text after `open` left unterminated swallowed the `}` that would close it:
     * a string whose unterminated rest holds a `}`, or, at the end of the file, a block
     * comment that ran to it.
     */
    pub(super) fn left_open_since(&self, open: Span) -> bool {
        let before = self.tokens.get(..self.pos).unwrap_or(&[]);
        let string = before
            .iter()
            .rev()
            .take_while(|t| t.span.lo > open.lo)
            .filter(|t| t.kind == TokenKind::Error)
            .map(|t| self.text_of(*t))
            .any(|s| s.starts_with('"') && (s.len() == 1 || !s.ends_with('"')) && s.contains('}'));
        let comment =
            self.at(TokenKind::Eof) && self.comment_to_eof().is_some_and(|c| c.lo > open.lo);
        string || comment
    }

    /** The block comment that runs unterminated to the end of the file, if there is one. */
    pub(in crate::compiler::syntax::parse) fn comment_to_eof(&self) -> Option<Span> {
        /* An unterminated block comment runs to the end, so it is the file's last comment. */
        let c = self.comments.last()?;
        let s = self
            .text
            .get(c.span.lo as usize..c.span.hi as usize)
            .unwrap_or("");
        (s.starts_with("/*") && !s.ends_with("*/")).then_some(c.span)
    }
}
