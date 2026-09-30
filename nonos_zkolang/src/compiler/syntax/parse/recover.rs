/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Recovery. A statement or item loop that receives `Err` resynchronizes by skipping to a
 * point where parsing can resume, so one run reports many errors, and every loop consumes
 * at least one token per turn, so parsing always terminates. This file holds the skip both
 * levels share and the recovery inside a block.
 */

use super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Skip tokens until one of `stops` at the current bracket depth, or a closing bracket
     * that would take the depth below zero. Brackets in between are skipped whole, without
     * recursion, so recovery costs no stack however deep the skipped text nests.
     */
    pub(super) fn skip_until(&mut self, stops: &[TokenKind]) {
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
                    if depth == 0 && stops.contains(&k) {
                        self.bump();
                        return;
                    }
                }
                _ if depth == 0 && stops.contains(&k) => return,
                _ => {}
            }
            self.bump();
        }
    }

    /**
     * Recover inside a block: skip past the next `;` at this depth, or stop before the
     * closing `}`.
     */
    pub(super) fn recover_stmt(&mut self) {
        let start = self.pos;
        self.skip_until(&[TokenKind::Semi]);
        if !self.eat(TokenKind::Semi)
            && self.pos == start
            && !matches!(self.kind(), TokenKind::RBrace | TokenKind::Eof)
        {
            /* A stray closing bracket: step over it so the block loop makes progress. */
            self.bump();
        }
    }
}
