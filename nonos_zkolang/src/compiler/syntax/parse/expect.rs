/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Expectation: consuming a token that must be there, or reporting what was found. */

use alloc::format;
use alloc::string::String;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::syntax::ast::Ident;
use crate::compiler::syntax::token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /** Consume a token that must be `k`, reporting what was found otherwise. */
    pub(super) fn expect(&mut self, k: TokenKind) -> PResult<Token> {
        if self.at(k) {
            return Ok(self.bump());
        }
        let ends = matches!(
            k,
            TokenKind::Semi
                | TokenKind::Comma
                | TokenKind::RParen
                | TokenKind::RBracket
                | TokenKind::RBrace
                | TokenKind::LBrace
        );
        Err(self.report_unexpected(k.describe(), ends))
    }

    /** Consume `close` after a list element that no comma followed. */
    pub(super) fn expect_list_end(&mut self, close: TokenKind) -> PResult<Token> {
        if self.at(close) {
            return Ok(self.bump());
        }
        let expected = format!("`,` or {}", close.describe());
        Err(self.report_unexpected(&expected, true))
    }

    /** Report that the current token is not what was expected. */
    pub(super) fn unexpected(&mut self, expected: &str) -> Reported {
        self.report_unexpected(expected, false)
    }

    /** Consume an identifier. */
    pub(super) fn ident(&mut self) -> PResult<Ident> {
        if self.at(TokenKind::Ident) {
            let t = self.bump();
            return Ok(Ident {
                name: String::from(self.text_of(t)),
                span: t.span,
            });
        }
        Err(self.unexpected("an identifier"))
    }
}
