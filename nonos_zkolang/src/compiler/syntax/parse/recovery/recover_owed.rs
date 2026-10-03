/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Recovery that closes what an erroneous construct opened. Skipping forward to a fixed
 * token is not enough: a `;` inside a parenthesis the error left open is not the end of
 * the statement, and stopping there would leave the parenthesis to swallow everything
 * after it. So the skip first pays the closers the construct owes.
 */

use super::super::parser::Parser;
use super::owed::Owed;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** The closers owed by brackets opened from token `from` up to the current one. */
    fn owed_since(&self, from: usize) -> Owed {
        let mut owed = Owed::default();
        for t in self.tokens.get(from..self.pos).unwrap_or(&[]) {
            owed.take(t.kind);
        }
        owed
    }

    /**
     * Skip to the closers owed since token `from`, consuming them and whatever they
     * enclose. A closer that matches none of them belongs to a construct opened before
     * `from`, so the skip ends before it and leaves it to that construct. Inside a
     * parenthesis or bracket the error left open, a `;`, a `let` or an item keyword that
     * starts a line ends the skip: none of them can continue what the bracket began.
     */
    pub(in crate::compiler::syntax::parse) fn pay_owed(&mut self, from: usize) {
        self.split = None;
        if self.at(TokenKind::Eof) {
            return;
        }
        let mut owed = self.owed_since(from);
        let before = owed.len();
        while owed.len() != 0 {
            let k = self.kind();
            let owes = owed.last().unwrap_or(TokenKind::Eof);
            let left_open = owed.len() <= before;
            let paren = matches!(owes, TokenKind::RParen | TokenKind::RBracket);
            if k == TokenKind::Eof
                || (left_open && paren && self.ends_bracket(k))
                || (left_open && owes == TokenKind::RBrace && self.at_outer_item(false))
            {
                return;
            }
            /* In a file with a `{` missing, a `}` where a `)` is owed stands for it. */
            let stands_for = paren && k == TokenKind::RBrace && self.layout.missing_openers;
            if !owed.take(if stands_for { owes } else { k }) {
                return;
            }
            self.bump();
        }
    }
}
