/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Nested block comments, and the error for one that is never closed. */

use super::comment::{Comment, CommentKind};
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::{FileId, Span};

/** Scan the block comment at `i`, reporting it if it is never closed. */
pub(super) fn block_comment(
    b: &[u8],
    i: usize,
    len: usize,
    file: FileId,
    diags: &mut Diagnostics,
) -> (Comment, usize) {
    let (comment, next, closed) = scan_block_comment(b, i, len, file);
    if !closed {
        diags.push(
            Diagnostic::error(
                Code::UNTERMINATED_COMMENT,
                "unterminated block comment",
                Span::new(file, i as u32, (i + 2).min(len) as u32),
                "this comment is never closed",
            )
            .with_help("block comments nest: every `/*` needs its own `*/`"),
        );
    }
    (comment, next)
}

/**
 * Scan a nested block comment starting at `i`. Returns the comment, the offset after it,
 * and whether it was closed before the end of the file.
 */
fn scan_block_comment(b: &[u8], i: usize, len: usize, file: FileId) -> (Comment, usize, bool) {
    let mut depth = 0usize;
    let mut j = i;
    let mut closed = false;
    while j < len {
        if b[j] == b'/' && b.get(j + 1) == Some(&b'*') && j + 1 < len {
            depth += 1;
            j += 2;
        } else if b[j] == b'*' && b.get(j + 1) == Some(&b'/') && j + 1 < len {
            depth = depth.saturating_sub(1);
            j += 2;
            if depth == 0 {
                closed = true;
                break;
            }
        } else {
            j += 1;
        }
    }
    let span = Span::new(file, i as u32, j as u32);
    /*
     * A second star after the opening slash and star makes an outer doc comment and a bang
     * an inner one; a third star, or a comment closed at once, keeps it ordinary.
     */
    let kind = match (b.get(i + 2), b.get(i + 3)) {
        (Some(b'*'), Some(&c)) if c != b'*' && c != b'/' => CommentKind::DocOuter,
        (Some(b'!'), _) => CommentKind::DocInner,
        _ => CommentKind::Block,
    };
    (Comment { span, kind }, j, closed)
}
