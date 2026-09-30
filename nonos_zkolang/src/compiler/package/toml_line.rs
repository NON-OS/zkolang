/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One line of a manifest: `[name]`, `key = value`, or blank, each with an optional `#` comment. */

use super::toml::Entry;
use super::toml_cursor::{Cursor, Line};
use crate::compiler::diag::Diagnostic;

impl Cursor<'_> {
    /** The next line: a header, an entry, or `None` for a blank or comment line. */
    pub(super) fn line(&mut self) -> Result<Option<Line>, Diagnostic> {
        self.blank();
        let line = match self.peek() {
            None | Some(b'\n' | b'#') => None,
            Some(b'[') => {
                let lo = self.pos();
                self.bump();
                self.blank();
                let (name, _) = self.key()?;
                self.blank();
                self.expect(b']', "expected `]`")?;
                Some(Line::Header(name, self.span_from(lo)))
            }
            Some(_) => Some(Line::Entry(self.entry(true)?)),
        };
        self.end_line()?;
        Ok(line)
    }

    /** `key = value`; a value may be an inline table only if `nest`. */
    pub(super) fn entry(&mut self, nest: bool) -> Result<Entry, Diagnostic> {
        let (key, span) = self.key()?;
        self.blank();
        self.expect(b'=', "expected `=` after the key")?;
        self.blank();
        let (value, value_span) = self.value(nest)?;
        Ok(Entry {
            key,
            span,
            value,
            value_span,
        })
    }
}
