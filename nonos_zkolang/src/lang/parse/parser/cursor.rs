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
        match self.bump() {
            Some(t) if t == want => Ok(()),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }
}
