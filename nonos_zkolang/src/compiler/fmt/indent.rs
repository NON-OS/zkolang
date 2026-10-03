/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * How deep each line is indented: one level for each line that opened brackets still open
 * after the ones it closes first, and one more for a line that goes on with the
 * expression the line before it left unfinished.
 */

use alloc::vec::Vec;

use super::brackets::closes;
use super::line::{Line, Start};
use crate::compiler::syntax::TokenKind;

/**
 * Whether a line starting with `first` goes on with the code line before it, which starts
 * with `before_first` and ends with `before_last`: not after a statement, a bracket, a
 * comma or an attribute, and not when it closes a bracket itself.
 */
fn goes_on(before: Option<&Line>, first: Option<TokenKind>) -> bool {
    let Some(b) = before else {
        return false;
    };
    let ends = matches!(
        b.last,
        Some(
            TokenKind::Semi
                | TokenKind::Comma
                | TokenKind::LBrace
                | TokenKind::RBrace
                | TokenKind::LParen
                | TokenKind::LBracket
        )
    );
    let attribute = b.first == Some(TokenKind::Pound) && b.last == Some(TokenKind::RBracket);
    !(ends || attribute || closes(first) || first == Some(TokenKind::Pound))
}

/** The level of each line; a line inside a comment or a string is placed by the formatter. */
pub(super) fn levels(lines: &[Line]) -> Vec<usize> {
    let mut before: Option<&Line> = None;
    let mut out = Vec::with_capacity(lines.len());
    for l in lines {
        let level = match l.start {
            Start::Code => l.base.saturating_add(usize::from(goes_on(before, l.first))),
            Start::Comment => l.depth,
            Start::Blank | Start::InComment | Start::InString => 0,
        };
        if l.last.is_some() {
            before = Some(l);
        }
        out.push(level);
    }
    out
}
