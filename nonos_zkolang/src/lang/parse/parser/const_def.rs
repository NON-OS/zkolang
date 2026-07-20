/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A constant definition, scalar or table.

use alloc::vec;
use alloc::vec::Vec;

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::ConstDef;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// `const name = 5;` binds a scalar; `const name = [n0, ...];` binds a table. The
    /// `[` after `=` selects between them.
    pub(crate) fn const_def(&mut self) -> Result<ConstDef, CompileError> {
        self.pos += 1;
        let name = self.ident()?;
        self.expect(&Tok::Assign)?;
        if !matches!(self.peek(), Some(Tok::LBracket)) {
            let v = self.number()?;
            self.expect(&Tok::Semi)?;
            return Ok(ConstDef {
                name,
                values: vec![v],
                scalar: true,
            });
        }
        self.pos += 1;
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
        Ok(ConstDef {
            name,
            values,
            scalar: false,
        })
    }
}
