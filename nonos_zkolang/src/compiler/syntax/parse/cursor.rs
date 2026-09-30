/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The token cursor: the current token, lookahead, and the spans around the cursor. */

use super::parser::Parser;
use crate::compiler::source::Span;
use crate::compiler::syntax::token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /** The current token. */
    pub(super) fn tok(&self) -> Token {
        if let Some(t) = self.split {
            return t;
        }
        self.tokens
            .get(self.pos)
            .copied()
            .unwrap_or_else(|| self.eof_token())
    }

    fn eof_token(&self) -> Token {
        let end = u32::try_from(self.text.len()).unwrap_or(u32::MAX);
        Token {
            kind: TokenKind::Eof,
            span: Span::new(self.file, end, end),
        }
    }

    /** The current token's kind. */
    pub(super) fn kind(&self) -> TokenKind {
        self.tok().kind
    }

    /** The kind `n` tokens ahead, ignoring any pending split. */
    pub(super) fn peek(&self, n: usize) -> TokenKind {
        self.tokens
            .get(self.pos + n)
            .map(|t| t.kind)
            .unwrap_or(TokenKind::Eof)
    }

    /** The current token's span. */
    pub(super) fn span(&self) -> Span {
        self.tok().span
    }

    /** The span of the token just consumed, or the current one at the start. */
    pub(super) fn prev_span(&self) -> Span {
        if self.pos == 0 {
            return self.span();
        }
        self.tokens
            .get(self.pos - 1)
            .map(|t| t.span)
            .unwrap_or_else(|| self.span())
    }
}
