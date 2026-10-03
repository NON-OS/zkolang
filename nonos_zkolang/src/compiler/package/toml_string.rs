/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A manifest string, `"text"`, with the escapes `\\"`, `\\\\`, `\\n` and `\\t`. */

use alloc::string::String;
use alloc::vec::Vec;

use super::toml_cursor::Cursor;
use crate::compiler::diag::Diagnostic;

impl Cursor<'_> {
    pub(super) fn string(&mut self) -> Result<String, Diagnostic> {
        self.bump();
        let mut out = Vec::new();
        loop {
            match self.peek() {
                None | Some(b'\n') => return Err(self.error("the string is not closed")),
                Some(b'"') => break,
                Some(b'\\') => {
                    self.bump();
                    out.push(match self.peek() {
                        Some(b'"') => b'"',
                        Some(b'\\') => b'\\',
                        Some(b'n') => b'\n',
                        Some(b't') => b'\t',
                        _ => {
                            return Err(self.error(
                                "unknown escape; `\\\"`, `\\\\`, `\\n` and `\\t` are known",
                            ))
                        }
                    });
                }
                Some(b) => out.push(b),
            }
            self.bump();
        }
        self.bump();
        Ok(String::from_utf8_lossy(&out).into_owned())
    }
}
