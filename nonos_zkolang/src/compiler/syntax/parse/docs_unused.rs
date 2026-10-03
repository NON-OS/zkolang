/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The warning for documentation that documents nothing. A run of doc comments of one
 * kind, with only whitespace between them, is one block and gets one warning. A file with
 * errors gets none: recovery skips code, and the documentation of what it skipped would
 * be reported as documenting nothing.
 */

use alloc::vec::Vec;

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::lex::CommentKind;

impl<'a> Parser<'a> {
    /** Warn once for each block of doc comments that no item or module took. */
    pub(super) fn warn_unused_docs(&mut self) {
        let file = self.file;
        if self
            .diags
            .items()
            .iter()
            .any(|d| d.is_error() && d.span().file == file)
        {
            return;
        }
        let unused = |i: usize| !self.doc_used.get(i).copied().unwrap_or(true);
        let mut blocks = Vec::new();
        let mut i = 0;
        while let Some(c) = self.comments.get(i) {
            i += 1;
            if !matches!(c.kind, CommentKind::DocOuter | CommentKind::DocInner) || !unused(i - 1) {
                continue;
            }
            let mut last = c.span;
            while let Some(n) = self.comments.get(i) {
                let gap = self
                    .text
                    .get(last.hi as usize..n.span.lo as usize)
                    .unwrap_or("x");
                if n.kind != c.kind || !unused(i) || !gap.trim().is_empty() {
                    break;
                }
                last = n.span;
                i += 1;
            }
            blocks.push((c.kind, c.span.to(last)));
        }
        for (kind, at) in blocks {
            let (label, help) = if kind == CommentKind::DocInner {
                ("not at the start of a module or file", "`/*! */` and `//!` document the module they open, before its first item; use `/* */` or `//` for other comments")
            } else {
                ("no item follows it", "`/** */` and `///` document the item after them; use `/* */` or `//` for other comments")
            };
            let d = Diagnostic::warning(
                Code::MISPLACED_DOC,
                "doc comment documents nothing",
                at,
                label,
            );
            self.diags.push(d.with_help(help));
        }
    }
}
