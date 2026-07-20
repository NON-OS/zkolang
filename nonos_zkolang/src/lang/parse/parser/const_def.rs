/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A constant table definition.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::ConstDef;
use crate::lang::CompileError;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// `const name = [n0, n1, ...];`: decimal literals in declaration order, later
    /// read by a compile-time index.
    pub(crate) fn const_def(&mut self) -> Result<ConstDef, CompileError> {
        self.pos += 1;
        let name = self.ident()?;
        self.expect(&Tok::Assign)?;
        self.expect(&Tok::LBracket)?;
        let mut values = Vec::new();
        if !matches!(self.peek(), Some(Tok::RBracket)) {
            loop {
                values.push(self.number()?);
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RBracket)?;
        self.expect(&Tok::Semi)?;
        Ok(ConstDef { name, values })
    }
}
