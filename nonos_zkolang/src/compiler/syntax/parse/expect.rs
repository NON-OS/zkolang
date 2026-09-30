/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Expectation: consuming a token that must be there, or reporting what was found. */

use alloc::format;
use alloc::string::String;

use super::parser::{PResult, Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Ident;
use crate::compiler::syntax::token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /** Consume a token that must be `k`, reporting what was found otherwise. */
    pub(super) fn expect(&mut self, k: TokenKind) -> PResult<Token> {
        if self.at(k) {
            return Ok(self.bump());
        }
        Err(self.unexpected(k.describe()))
    }

    /** Report that the current token is not what was expected. */
    pub(super) fn unexpected(&mut self, expected: &str) -> Reported {
        let t = self.tok();
        if t.kind == TokenKind::Error || self.at_reserved() || self.after_stray() {
            /* The lexer already reported this text, or text just before it. */
            return Reported;
        }
        let found = match t.kind {
            TokenKind::Ident => format!("`{}`", self.text_of(t)),
            k => String::from(k.describe()),
        };
        let message = format!("expected {expected}, found {found}");
        /*
         * What is missing at the end of a line belongs after the line's last token, so the
         * report points there and marks the token found on a later line as a second label.
         */
        let d = match self.line_end_before(t.span) {
            Some(end) => Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                message,
                end,
                format!("expected {expected} after this"),
            )
            .with_label(t.span, format!("found {found}")),
            None => Diagnostic::error(
                Code::UNEXPECTED_TOKEN,
                message,
                t.span,
                format!("expected {expected}"),
            ),
        };
        self.diags.push(d);
        Reported
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
