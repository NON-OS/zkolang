/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The closers a recovery skip owes: a stack of them, with a count of each kind so that a
 * closer matching none is known at once. Every closer is pushed and popped at most once,
 * so a skip costs time linear in the tokens it passes.
 */

use alloc::vec::Vec;

use crate::compiler::syntax::token::TokenKind;

/** Closers owed, innermost last. */
#[derive(Default)]
pub(super) struct Owed {
    stack: Vec<TokenKind>,
    counts: [usize; 3],
}

/** The slot of a closer in the counts, and the closer an opener is owed. */
fn slot(k: TokenKind) -> Option<(usize, TokenKind)> {
    match k {
        TokenKind::RParen | TokenKind::LParen => Some((0, TokenKind::RParen)),
        TokenKind::RBracket | TokenKind::LBracket => Some((1, TokenKind::RBracket)),
        TokenKind::RBrace | TokenKind::LBrace => Some((2, TokenKind::RBrace)),
        _ => None,
    }
}

impl Owed {
    /**
     * Account for token `k`: an opener owes its closer, and a closer pays the innermost
     * one of its kind with everything opened inside it. Returns false, changing nothing,
     * for a closer owed nowhere.
     */
    pub(super) fn take(&mut self, k: TokenKind) -> bool {
        let Some((s, c)) = slot(k) else {
            return true;
        };
        if c != k {
            self.counts[s] += 1;
            self.stack.push(c);
            return true;
        }
        if self.counts[s] == 0 {
            return false;
        }
        while let Some(top) = self.stack.pop() {
            if let Some((t, _)) = slot(top) {
                self.counts[t] -= 1;
            }
            if top == k {
                break;
            }
        }
        true
    }

    /** How many closers are owed. */
    pub(super) fn len(&self) -> usize {
        self.stack.len()
    }

    /** The innermost closer owed. */
    pub(super) fn last(&self) -> Option<TokenKind> {
        self.stack.last().copied()
    }
}
