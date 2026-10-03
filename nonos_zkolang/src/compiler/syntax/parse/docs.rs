/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Doc comments. An item's documentation is the run of outer doc comments between the
 * token before it and its first token. A module's is the run of inner doc comments at the
 * start of its file or inline body. A doc comment that documents nothing is a warning.
 */

use alloc::string::String;

use super::doc_text::doc_text;
use super::parser::Parser;
use crate::compiler::syntax::lex::CommentKind;

impl<'a> Parser<'a> {
    /** The outer doc comments directly before offset `before`: those after the previous token. */
    pub(super) fn take_doc(&mut self, before: u32) -> Option<String> {
        let prev = self.pos.checked_sub(1).and_then(|i| self.tokens.get(i));
        let after = prev.map_or(0, |t| t.span.hi);
        self.take_comments(CommentKind::DocOuter, after, before)
    }

    /** The inner doc comments between two offsets. */
    pub(super) fn take_inner_doc(&mut self, after: u32, before: u32) -> Option<String> {
        self.take_comments(CommentKind::DocInner, after, before)
    }

    fn take_comments(&mut self, kind: CommentKind, after: u32, before: u32) -> Option<String> {
        let mut out: Option<String> = None;
        /* Comments are in source order: find the first after `after`, and stop at `before`. */
        let first = self.comments.partition_point(|c| c.span.lo < after);
        for (i, c) in self.comments.iter().enumerate().skip(first) {
            if c.span.lo >= before {
                break;
            }
            if c.kind != kind {
                continue;
            }
            if let Some(used) = self.doc_used.get_mut(i) {
                *used = true;
            }
            let text = self
                .text
                .get(c.span.lo as usize..c.span.hi as usize)
                .unwrap_or("");
            let s = out.get_or_insert_with(String::new);
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(&doc_text(text));
        }
        out
    }
}
