/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Consume a numeric literal, for a loop bound or a table entry.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// Consume a single decimal literal.
    pub(crate) fn number(&mut self) -> Result<u64, CompileError> {
        match self.bump() {
            Some(Tok::Num(v)) => Ok(*v),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }
}
