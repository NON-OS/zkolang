/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The token cursor: peek, advance, and expect.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// The token at the cursor, without advancing.
    pub(crate) fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    /// The byte offset of the token at the cursor, or the end of the source when the
    /// cursor is past the last token, so an error can point at where it occurred.
    pub(crate) fn at(&self) -> usize {
        self.spans.get(self.pos).copied().unwrap_or(self.eof)
    }

    /// The token at the cursor, advancing past it.
    pub(crate) fn bump(&mut self) -> Option<&Tok> {
        let t = self.toks.get(self.pos);
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    /// Consume a token that must be exactly `want`.
    pub(crate) fn expect(&mut self, want: &Tok) -> Result<(), CompileError> {
        let at = self.at();
        match self.bump() {
            Some(t) if t == want => Ok(()),
            Some(_) => Err(CompileError::UnexpectedToken { at }),
            None => Err(CompileError::UnexpectedEof { at }),
        }
    }
}
