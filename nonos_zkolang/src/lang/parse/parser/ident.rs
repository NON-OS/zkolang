/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Consume an identifier and a numeric literal.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::CompileError;
use alloc::string::String;

impl<'a> Parser<'a> {
    /// Consume an identifier, for a name or a parameter.
    pub(crate) fn ident(&mut self) -> Result<String, CompileError> {
        match self.bump() {
            Some(Tok::Ident(n)) => Ok(n.clone()),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }
}
