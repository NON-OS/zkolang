/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The pieces of a manifest line: a bare key, an expected character, and the line's end. */

use alloc::string::String;
use alloc::vec::Vec;

use super::toml_cursor::Cursor;
use crate::compiler::diag::Diagnostic;
use crate::compiler::source::Span;

impl Cursor<'_> {
    /** A bare key: ASCII letters, digits, `_` and `-`. */
    pub(super) fn key(&mut self) -> Result<(String, Span), Diagnostic> {
        let lo = self.pos();
        let mut out = Vec::new();
        while let Some(b) = self
            .peek()
            .filter(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
        {
            out.push(b);
            self.bump();
        }
        if out.is_empty() {
            return Err(self.error("expected a key"));
        }
        Ok((
            String::from_utf8_lossy(&out).into_owned(),
            self.span_from(lo),
        ))
    }

    /** The character `b`, or the error `what`. */
    pub(super) fn expect(&mut self, b: u8, what: &str) -> Result<(), Diagnostic> {
        if self.peek() != Some(b) {
            return Err(self.error(what));
        }
        self.bump();
        Ok(())
    }

    /** The end of a line: blanks, then a comment, a line break or the end of the text. */
    pub(super) fn end_line(&mut self) -> Result<(), Diagnostic> {
        self.blank();
        match self.peek() {
            None => Ok(()),
            Some(b'\n' | b'#') => {
                self.skip_line();
                Ok(())
            }
            Some(_) => Err(self.error("expected the end of the line")),
        }
    }
}
