/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A line of a file as the formatter reads it. */

use crate::compiler::syntax::TokenKind;

/** What a line starts with. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Start {
    Blank,
    Code,
    Comment,
    /** Inside a block comment that began on an earlier line. */
    InComment,
    /** Inside a string that began on an earlier line. */
    InString,
}

/** One line: the bytes `lo..hi` of the text, the line break left out. */
#[derive(Clone, Copy, Debug)]
pub(super) struct Line {
    pub(super) lo: usize,
    pub(super) hi: usize,
    pub(super) start: Start,
    pub(super) first: Option<TokenKind>,
    pub(super) last: Option<TokenKind>,
    /** How many lines opened the brackets open before it. */
    pub(super) depth: usize,
    /** The same, with the brackets it closes before any other token left out. */
    pub(super) base: usize,
    /** Whether a comment or a string goes on past the line's end. */
    pub(super) open_end: bool,
}
