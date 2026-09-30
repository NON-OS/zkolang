/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Comments. They are not tokens, but the formatter and the documentation tool need them,
 * so the lexer keeps each with its span and kind.
 */

use crate::compiler::source::{FileId, Span};

/** What a comment is. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CommentKind {
    /** Two slashes to the end of the line. */
    Line,
    /** A slash and a star to a star and a slash, possibly nested. */
    Block,
    /** Three slashes, or a block with a second star, documenting the item that follows. */
    DocOuter,
    /** Two slashes and a bang, or a block with a bang, documenting the enclosing module. */
    DocInner,
}

/** A comment and where it is. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Comment {
    pub span: Span,
    pub kind: CommentKind,
}

/**
 * Scan a line comment starting at `i`, returning it and the offset of the line end. A
 * line ends at a line feed or a carriage return, as editors display it.
 */
pub(super) fn scan_line_comment(b: &[u8], i: usize, len: usize, file: FileId) -> (Comment, usize) {
    let mut j = i;
    while j < len && b[j] != b'\n' && b[j] != b'\r' {
        j += 1;
    }
    let kind = if b.get(i + 2) == Some(&b'/') && b.get(i + 3) != Some(&b'/') {
        CommentKind::DocOuter
    } else if b.get(i + 2) == Some(&b'!') {
        CommentKind::DocInner
    } else {
        CommentKind::Line
    };
    let span = Span::new(file, i as u32, j as u32);
    (Comment { span, kind }, j)
}
