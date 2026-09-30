/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Recovery from an error in a function's signature: its body is still parsed. */

use super::super::parser::Parser;
use super::recover_item::starts_item;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Skip the rest of a signature that failed to parse to the `{` of its body, and parse
     * the body so the errors in it are reported.
     */
    pub(in crate::compiler::syntax::parse) fn body_after_bad_signature(&mut self) {
        if self.skip_to_brace() {
            let _ = self.block();
        }
    }

    /**
     * Skip to the next `{` outside parentheses and brackets, and say whether there is one.
     * The skip ends without one at a `;`, at an item that starts a line, at a token after
     * the first that can begin an item, or at the end.
     */
    pub(in crate::compiler::syntax::parse) fn skip_to_brace(&mut self) -> bool {
        self.split = None;
        let first = self.pos;
        let mut depth: usize = 0;
        loop {
            match self.kind() {
                TokenKind::Eof => return false,
                TokenKind::LBrace if depth == 0 => return true,
                TokenKind::Semi if depth == 0 => return false,
                TokenKind::LParen | TokenKind::LBracket => depth += 1,
                TokenKind::RParen | TokenKind::RBracket => depth = depth.saturating_sub(1),
                _ if self.item_starts_line() => return false,
                k if depth == 0 && self.pos != first && starts_item(k) => return false,
                _ => {}
            }
            self.bump();
        }
    }
}
