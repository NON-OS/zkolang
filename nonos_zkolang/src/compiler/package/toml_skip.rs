/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Skipping blanks and lines, and reporting at the current character. */

use super::toml_cursor::Cursor;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

impl Cursor<'_> {
    /** A manifest error (E0902) at the current character. */
    pub(super) fn error(&self, what: &str) -> Diagnostic {
        let hi = self.pos().saturating_add(u32::from(!self.done()));
        let at = Span::new(self.file, self.pos(), hi);
        Diagnostic::error(Code::MANIFEST, what, at, "here")
    }

    /** Skip spaces and tabs. */
    pub(super) fn blank(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\r')) {
            self.bump();
        }
    }

    /** Skip to the start of the next line. */
    pub(super) fn skip_line(&mut self) {
        while let Some(b) = self.peek() {
            self.bump();
            if b == b'\n' {
                break;
            }
        }
    }
}
