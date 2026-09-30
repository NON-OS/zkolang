/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Skipping the rest of a statement that failed to parse. */

use super::super::parser::Parser;
use super::recover_item::starts_item;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Skip to the `;` that ends the statement, or stop before a closing bracket of the
     * enclosing block, before a token that starts the next statement or an item, or before
     * a block-like statement that starts a line.
     * Brackets in between are skipped whole, without recursion.
     */
    pub(in crate::compiler::syntax::parse) fn skip_stmt_rest(&mut self) {
        self.split = None;
        let mut depth: usize = 0;
        loop {
            let k = self.kind();
            match k {
                TokenKind::Eof => return,
                TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                    if depth == 0 {
                        return;
                    }
                    depth -= 1;
                }
                TokenKind::Semi if depth == 0 => return,
                TokenKind::Kw(Keyword::Let | Keyword::Assert) if depth == 0 => return,
                /* These may go on an expression, but at the start of a line they start a statement. */
                TokenKind::Kw(
                    Keyword::If | Keyword::Match | Keyword::For | Keyword::While | Keyword::Return,
                ) if depth == 0 && self.line_end_before(self.span()).is_some() => return,
                _ if depth == 0 && starts_item(k) => return,
                _ => {}
            }
            self.bump();
        }
    }
}
