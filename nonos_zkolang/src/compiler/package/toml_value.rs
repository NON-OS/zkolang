/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A manifest value: a `"string"` with the escapes `\"`, `\\`, `\n` and `\t`, a decimal
 * integer that fits 64 bits, or an inline table `{ key = value, .. }` of strings and
 * integers, which does not nest.
 */

use alloc::vec::Vec;

use super::toml::{Entry, Value};
use super::toml_cursor::Cursor;
use crate::compiler::diag::Diagnostic;
use crate::compiler::source::Span;

impl Cursor<'_> {
    /** A value, which may be an inline table only if `nest`. */
    pub(super) fn value(&mut self, nest: bool) -> Result<(Value, Span), Diagnostic> {
        let lo = self.pos();
        let v = match self.peek() {
            Some(b'"') => Value::Str(self.string()?),
            Some(b'{') if nest => Value::Table(self.inline()?),
            Some(b'{') => return Err(self.error("an inline table holds no inline table")),
            Some(b) if b.is_ascii_digit() => Value::Int(self.int()?),
            _ => return Err(self.error("expected a string, an integer or `{ .. }`")),
        };
        Ok((v, self.span_from(lo)))
    }

    fn int(&mut self) -> Result<u64, Diagnostic> {
        let mut v: u64 = 0;
        while let Some(d) = self.peek().filter(u8::is_ascii_digit) {
            let digit = u64::from(d.saturating_sub(b'0'));
            let next = v.checked_mul(10).and_then(|v| v.checked_add(digit));
            v = next.ok_or_else(|| self.error("the integer does not fit 64 bits"))?;
            self.bump();
        }
        Ok(v)
    }

    fn inline(&mut self) -> Result<Vec<Entry>, Diagnostic> {
        self.bump();
        self.blank();
        let mut out = Vec::new();
        if self.peek() == Some(b'}') {
            self.bump();
            return Ok(out);
        }
        loop {
            out.push(self.entry(false)?);
            self.blank();
            match self.peek() {
                Some(b',') => {
                    self.bump();
                    self.blank();
                }
                Some(b'}') => break,
                _ => return Err(self.error("expected `,` or `}`")),
            }
        }
        self.bump();
        Ok(out)
    }
}
