/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A variable reference or a function call.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::string::String;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// An identifier is a call when followed by `(`, otherwise a variable.
    pub(crate) fn ident_expr(&mut self, name: String) -> Result<Expr, CompileError> {
        if !matches!(self.peek(), Some(Tok::LParen)) {
            return Ok(Expr::Var(name));
        }
        self.pos += 1;
        let mut args = Vec::new();
        if !matches!(self.peek(), Some(Tok::RParen)) {
            loop {
                args.push(self.expr()?);
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RParen)?;
        Ok(Expr::Call(name, args))
    }
}
