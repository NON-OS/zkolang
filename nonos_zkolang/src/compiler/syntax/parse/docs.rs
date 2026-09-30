/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Doc comments. An item's documentation is the run of `///` comments between the token
 * before it and its first token. A module's is the run of `//!` comments at the start of
 * its file or inline body. A doc comment that documents nothing is a warning.
 */

use alloc::string::String;

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::lex::CommentKind;

impl<'a> Parser<'a> {
    /** The `///` comments directly before offset `before`: those after the previous token. */
    pub(super) fn take_doc(&mut self, before: u32) -> Option<String> {
        let after = if self.pos == 0 {
            0
        } else {
            self.tokens
                .get(self.pos - 1)
                .map(|t| t.span.hi)
                .unwrap_or(0)
        };
        self.take_comments(CommentKind::DocOuter, after, before)
    }

    /** The `//!` comments between two offsets. */
    pub(super) fn take_inner_doc(&mut self, after: u32, before: u32) -> Option<String> {
        self.take_comments(CommentKind::DocInner, after, before)
    }

    fn take_comments(&mut self, kind: CommentKind, after: u32, before: u32) -> Option<String> {
        let mut out: Option<String> = None;
        for (i, c) in self.comments.iter().enumerate() {
            if c.kind != kind || c.span.lo < after || c.span.lo >= before {
                continue;
            }
            if let Some(used) = self.doc_used.get_mut(i) {
                *used = true;
            }
            let text = self
                .text
                .get(c.span.lo as usize..c.span.hi as usize)
                .unwrap_or("");
            let body = text.get(3..).unwrap_or("");
            let body = body.strip_prefix(' ').unwrap_or(body);
            let s = out.get_or_insert_with(String::new);
            if !s.is_empty() {
                s.push('\n');
            }
            s.push_str(body.trim_end());
        }
        out
    }

    /** Warn about every doc comment no item or module took. */
    pub(super) fn warn_unused_docs(&mut self) {
        for (i, c) in self.comments.iter().enumerate() {
            let doc = matches!(c.kind, CommentKind::DocOuter | CommentKind::DocInner);
            if doc && !self.doc_used.get(i).copied().unwrap_or(true) {
                self.diags.push(
                    Diagnostic::warning(Code::MISPLACED_DOC, "doc comment documents nothing", c.span, "")
                        .with_help("`///` documents the item after it and `//!` the module it opens; use `//` for other comments"),
                );
            }
        }
    }
}
