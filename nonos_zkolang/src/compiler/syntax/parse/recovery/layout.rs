/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Facts about the whole file that recovery reads: where each line starts and how far it
 * is indented, so either costs a search rather than a scan, and whether the braces balance,
 * so an item on its own line is taken to close a block only when some `}` is missing.
 */

use alloc::vec::Vec;

use crate::compiler::syntax::token::{Token, TokenKind};

/** Line starts and brace balance of one file. */
pub(in crate::compiler::syntax::parse) struct Layout {
    line_starts: Vec<u32>,
    /** The spaces and tabs each line starts with. */
    indents: Vec<u32>,
    /** Whether the file has more `{` than `}`. */
    pub(in crate::compiler::syntax::parse) missing_closers: bool,
    /** Whether the file has more `}` than `{`. */
    pub(in crate::compiler::syntax::parse) missing_openers: bool,
}

impl Layout {
    /** The layout of `text`, lexed into `tokens`. */
    pub(in crate::compiler::syntax::parse) fn new(text: &str, tokens: &[Token]) -> Layout {
        let b = text.as_bytes();
        let mut line_starts = alloc::vec![0];
        for (i, &c) in b.iter().enumerate() {
            if c == b'\n' || (c == b'\r' && b.get(i + 1) != Some(&b'\n')) {
                line_starts.push(u32::try_from(i + 1).unwrap_or(u32::MAX));
            }
        }
        let indents = line_starts
            .iter()
            .map(|&s| {
                b.get(s as usize..)
                    .unwrap_or(&[])
                    .iter()
                    .take_while(|&&c| c == b' ' || c == b'\t')
                    .count() as u32
            })
            .collect();
        let count = |k: TokenKind| tokens.iter().filter(|t| t.kind == k).count();
        let (open, close) = (count(TokenKind::LBrace), count(TokenKind::RBrace));
        Layout {
            line_starts,
            indents,
            missing_closers: open > close,
            missing_openers: close > open,
        }
    }

    /** The start of the line that holds `at`, and the indentation of that line. */
    pub(in crate::compiler::syntax::parse) fn line_of(&self, at: u32) -> (u32, u32) {
        let i = self
            .line_starts
            .partition_point(|&s| s <= at)
            .saturating_sub(1);
        let start = self.line_starts.get(i).copied().unwrap_or(0);
        (start, self.indents.get(i).copied().unwrap_or(0))
    }
}
