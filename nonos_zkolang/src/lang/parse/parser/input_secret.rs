/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The input and secret binding statements.

use super::Parser;
use crate::lang::parse::ast::Stmt;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// `input name;`: bind a name to the next public input.
    pub(crate) fn input_stmt(&mut self) -> Result<Stmt, CompileError> {
        self.pos += 1;
        let name = self.ident()?;
        self.expect(&crate::lang::lex::Tok::Semi)?;
        Ok(Stmt::Input(name))
    }

    /// `secret name;`: bind a name to the next private input.
    pub(crate) fn secret_stmt(&mut self) -> Result<Stmt, CompileError> {
        self.pos += 1;
        let name = self.ident()?;
        self.expect(&crate::lang::lex::Tok::Semi)?;
        Ok(Stmt::Secret(name))
    }
}
