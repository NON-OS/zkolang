/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A function definition.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::FnDef;
use crate::lang::CompileError;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// `fn name(a, b) = expr;`: the body is one expression, inlined at each call, so
    /// there is no statement block and no return keyword.
    pub(crate) fn fn_def(&mut self) -> Result<FnDef, CompileError> {
        self.pos += 1;
        let name = self.ident()?;
        self.expect(&Tok::LParen)?;
        let mut params = Vec::new();
        if !matches!(self.peek(), Some(Tok::RParen)) {
            loop {
                params.push(self.ident()?);
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RParen)?;
        self.expect(&Tok::Assign)?;
        let body = self.expr()?;
        self.expect(&Tok::Semi)?;
        Ok(FnDef { name, params, body })
    }
}
