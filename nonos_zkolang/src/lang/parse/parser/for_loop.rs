/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A bounded loop.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Stmt;
use crate::lang::CompileError;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// `for i in lo .. hi { stmt* }`: the bounds are literals, so the iteration count
    /// is known at compile time and the compiler unrolls it.
    pub(crate) fn for_loop(&mut self) -> Result<Stmt, CompileError> {
        self.pos += 1;
        let var = self.ident()?;
        self.expect(&Tok::In)?;
        let lo = self.number()?;
        self.expect(&Tok::DotDot)?;
        let hi = self.number()?;
        self.expect(&Tok::LBrace)?;
        let mut body = Vec::new();
        while !matches!(self.peek(), Some(Tok::RBrace)) {
            if self.peek().is_none() {
                return Err(CompileError::UnexpectedEof);
            }
            body.push(self.stmt()?);
        }
        self.expect(&Tok::RBrace)?;
        Ok(Stmt::For { var, lo, hi, body })
    }
}
