/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Consuming tokens, testing the current one, and reading a token's text. */

use super::parser::Parser;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /** Consume the current token. */
    pub(super) fn bump(&mut self) -> Token {
        if let Some(t) = self.split.take() {
            return t;
        }
        let t = self.tok();
        if self.pos < self.tokens.len() && t.kind != TokenKind::Eof {
            self.pos += 1;
        }
        t
    }

    /** Whether the current token is `k`. */
    pub(super) fn at(&self, k: TokenKind) -> bool {
        self.kind() == k
    }

    /** Whether the current token is keyword `k`. */
    pub(super) fn at_kw(&self, k: Keyword) -> bool {
        self.kind() == TokenKind::Kw(k)
    }

    /** Consume the current token if it is `k`. */
    pub(super) fn eat(&mut self, k: TokenKind) -> bool {
        if self.at(k) {
            self.bump();
            true
        } else {
            false
        }
    }

    /** Consume keyword `k` if it is current. */
    pub(super) fn eat_kw(&mut self, k: Keyword) -> bool {
        self.eat(TokenKind::Kw(k))
    }

    /** The text of a token. */
    pub(super) fn text_of(&self, t: Token) -> &'a str {
        self.text
            .get(t.span.lo as usize..t.span.hi as usize)
            .unwrap_or("")
    }
}
