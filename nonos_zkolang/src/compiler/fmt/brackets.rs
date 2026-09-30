/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The brackets open before each line. A line is indented once for each line that opened
 * brackets still open, so a line that opens two, as `f(match x {`, indents what follows
 * once, and the line that closes both goes back to where it began.
 */

use alloc::vec::Vec;

use crate::compiler::syntax::TokenKind;

/** Whether `k` closes a bracket. */
pub(super) fn closes(k: Option<TokenKind>) -> bool {
    matches!(
        k,
        Some(TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace)
    )
}

/** The line that opened each bracket open now, the innermost last. */
#[derive(Default)]
pub(super) struct Open(Vec<usize>);

impl Open {
    /** How many lines opened the brackets open now, the innermost `closed` left out. */
    pub(super) fn levels(&self, closed: usize) -> usize {
        let open = self
            .0
            .get(..self.0.len().saturating_sub(closed))
            .unwrap_or(&[]);
        let changes = open.windows(2).filter(|w| w.first() != w.last()).count();
        changes.saturating_add(usize::from(!open.is_empty()))
    }

    /** Open and close the brackets of `code`, the tokens of line `at`. */
    pub(super) fn take(&mut self, at: usize, code: &[TokenKind]) {
        for &k in code {
            match k {
                TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => self.0.push(at),
                _ if closes(Some(k)) => {
                    self.0.pop();
                }
                _ => {}
            }
        }
    }
}
