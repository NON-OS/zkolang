/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A single statement.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Stmt;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// One statement, dispatched on its leading keyword. Names reuse the identifier
    /// reader, so each binding arm stays short.
    pub(crate) fn stmt(&mut self) -> Result<Stmt, CompileError> {
        match self.peek() {
            Some(Tok::Let) => {
                self.pos += 1;
                let name = self.ident()?;
                self.expect(&Tok::Assign)?;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Let(name, e))
            }
            Some(Tok::Assert) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Assert(e))
            }
            Some(Tok::Input) => self.input_stmt(),
            Some(Tok::Secret) => self.secret_stmt(),
            Some(Tok::Output) => {
                self.pos += 1;
                let e = self.expr()?;
                self.expect(&Tok::Semi)?;
                Ok(Stmt::Output(e))
            }
            Some(Tok::For) => self.for_loop(),
            Some(_) => Err(CompileError::UnexpectedToken { at: self.at() }),
            None => Err(CompileError::UnexpectedEof { at: self.at() }),
        }
    }
}
