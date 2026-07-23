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
                if matches!(self.peek(), Some(Tok::LParen)) {
                    // `let (x, y, ...) = e;` destructures the tuple e produces into names.
                    self.pos += 1;
                    let mut names = alloc::vec::Vec::new();
                    loop {
                        names.push(self.ident()?);
                        if matches!(self.peek(), Some(Tok::Comma)) {
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    self.expect(&Tok::RParen)?;
                    self.expect(&Tok::Assign)?;
                    let e = self.expr()?;
                    self.expect(&Tok::Semi)?;
                    return Ok(Stmt::LetTuple(names, e));
                }
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
