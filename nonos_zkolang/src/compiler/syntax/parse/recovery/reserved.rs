/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Reserved words and the edition 2025 textual include. The lexer reports a reserved word;
 * the parser does not report it a second time, and skips the construct it begins.
 */

use super::super::parser::Parser;
use crate::compiler::syntax::keyword::RESERVED;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Whether the current token is a reserved word the lexer has reported. */
    pub(in crate::compiler::syntax::parse) fn at_reserved(&self) -> bool {
        self.at(TokenKind::Ident)
            && RESERVED.contains(&self.text_of(self.tok()))
            && !self.at_include()
    }

    /**
     * Skip a statement that a textual include or a reserved word begins, and say whether
     * there was one. After a reserved word the statement is a braced body, or runs to `;`.
     */
    pub(in crate::compiler::syntax::parse) fn skip_reserved_stmt(&mut self) -> bool {
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
