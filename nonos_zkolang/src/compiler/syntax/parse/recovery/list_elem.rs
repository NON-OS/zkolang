/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Recovery within a list of parameters, fields or variants: an error skips one element. */

use super::super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * After an error in a list element that began at token `from`, close what the element
     * opened and skip to the `,` that ends it, which is consumed, or to the list's `close`.
     * Says whether the list goes on.
     */
    pub(in crate::compiler::syntax::parse) fn recover_elem(
        &mut self,
        from: usize,
        close: TokenKind,
    ) -> bool {
        self.pay_owed(from);
        self.skip_elem(close) && (self.at(close) || self.eat(TokenKind::Comma))
    }

    /**
     * Skip to the `,` or `close` that ends a list element, and say whether one was found.
     * The skip ends without one at a `;`, at an item that starts a line, at a closer of an
     * enclosing construct, at a `{` outside a braced list, or at the end.
     */
    pub(in crate::compiler::syntax::parse) fn skip_elem(&mut self, close: TokenKind) -> bool {
        self.split = None;
        let mut depth: usize = 0;
        loop {
            let k = self.kind();
            match k {
                TokenKind::Eof => return false,
                _ if depth == 0 && (k == close || k == TokenKind::Comma) => return true,
                TokenKind::Semi if depth == 0 => return false,
                TokenKind::LBrace if depth == 0 && close != TokenKind::RBrace => return false,
                TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                    if depth == 0 {
                        return false;
                    }
                    depth -= 1;
                }
                _ if depth == 0 && self.item_starts_line() => return false,
                _ => {}
            }
            self.bump();
        }
    }
}
