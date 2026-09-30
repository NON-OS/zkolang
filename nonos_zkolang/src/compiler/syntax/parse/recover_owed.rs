/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The brackets an error leaves open. An error deep inside a statement or item unwinds past
 * the brackets its enclosing constructs had opened, so before recovery looks for a place
 * to resume, it skips to the closers those brackets are owed. Without this every closer
 * left over reads as a stray one and is reported again.
 */

use alloc::vec::Vec;

use super::parser::Parser;
use crate::compiler::syntax::token::TokenKind;

/** The closer an opening bracket is owed, if the token opens one. */
fn closer(k: TokenKind) -> Option<TokenKind> {
    match k {
        TokenKind::LParen => Some(TokenKind::RParen),
        TokenKind::LBracket => Some(TokenKind::RBracket),
        TokenKind::LBrace => Some(TokenKind::RBrace),
        _ => None,
    }
}

impl<'a> Parser<'a> {
    /** The closers owed by brackets opened from token `from` up to the current one. */
    fn owed_since(&self, from: usize) -> Vec<TokenKind> {
        let mut owed = Vec::new();
        for t in self.tokens.get(from..self.pos).unwrap_or(&[]) {
            if let Some(c) = closer(t.kind) {
                owed.push(c);
            } else if let Some(i) = owed.iter().rposition(|&c| c == t.kind) {
                owed.truncate(i);
            }
        }
        owed
    }

    /**
     * Skip to the closers owed since token `from`, consuming them and whatever they
     * enclose. A closer that matches none of them belongs to a construct opened before
     * `from`, so the skip ends before it and leaves it to that construct.
     */
    pub(super) fn pay_owed(&mut self, from: usize) {
        self.split = None;
        let mut owed = self.owed_since(from);
        while !owed.is_empty() {
            let k = self.kind();
            if k == TokenKind::Eof {
                return;
            }
            if let Some(c) = closer(k) {
                owed.push(c);
            } else if let Some(i) = owed.iter().rposition(|&c| c == k) {
                owed.truncate(i);
            } else if matches!(
                k,
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace
            ) {
                return;
            }
            self.bump();
        }
    }
}
