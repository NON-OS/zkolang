/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Recovery from an error in a function's signature: its body is still parsed. */

use super::super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Skip the rest of a signature that failed to parse to the `{` of its body, and parse
     * the body so the errors in it are reported. The skip ends without a body at a `;`, at
     * an item that starts a line, or at the end of the file.
     */
    pub(in crate::compiler::syntax::parse) fn body_after_bad_signature(&mut self) {
        self.split = None;
        let mut depth: usize = 0;
        loop {
            match self.kind() {
                TokenKind::Eof => return,
                TokenKind::LBrace if depth == 0 => break,
                TokenKind::Semi if depth == 0 => return,
                TokenKind::LParen | TokenKind::LBracket => depth += 1,
                TokenKind::RParen | TokenKind::RBracket => depth = depth.saturating_sub(1),
                _ if self.at_outer_item() => return,
                _ => {}
            }
            self.bump();
        }
        let _ = self.block();
    }
}
