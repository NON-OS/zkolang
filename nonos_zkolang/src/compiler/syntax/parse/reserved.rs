/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Reserved words and the edition 2025 textual include. The lexer reports a reserved word;
 * the parser does not report it a second time, and skips the construct it begins.
 */

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::keyword::RESERVED;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Whether the current token is a reserved word the lexer has reported. */
    pub(super) fn at_reserved(&self) -> bool {
        self.at(TokenKind::Ident)
            && RESERVED.contains(&self.text_of(self.tok()))
            && !self.at_include()
    }

    /** Whether the current token begins a textual include: `include "file"`. */
    pub(super) fn at_include(&self) -> bool {
        self.at(TokenKind::Ident)
            && self.text_of(self.tok()) == "include"
            && matches!(self.peek(1), TokenKind::Str | TokenKind::Error)
    }

    /** Report a textual include and skip it with the `;` after it, if any. */
    pub(super) fn skip_include(&mut self) {
        let start = self.bump().span;
        let file = self.bump().span;
        self.diags.push(
            Diagnostic::error(
                Code::INCLUDE_REMOVED,
                "`include` is not part of edition 2026",
                start.to(file),
                "textual include",
            )
            .with_help(
                "declare the file as a module with `mod name;` and import its items with `use`",
            ),
        );
        self.eat(TokenKind::Semi);
    }

    /**
     * Skip a statement that a textual include or a reserved word begins, and say whether
     * there was one. After a reserved word the statement is a braced body, or runs to `;`.
     */
    pub(super) fn skip_reserved_stmt(&mut self) -> bool {
        if self.at_include() {
            self.skip_include();
            return true;
        }
        if !self.at_reserved() {
            return false;
        }
        self.bump();
        if self.at(TokenKind::LBrace) {
            self.skip_until(&[TokenKind::RBrace]);
        } else {
            self.skip_stmt_rest();
            self.eat(TokenKind::Semi);
        }
        true
    }
}
