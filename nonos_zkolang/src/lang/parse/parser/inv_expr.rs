/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The field-inverse builtin.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// `inv(e)`: the field inverse.
    pub(crate) fn inv_expr(&mut self) -> Result<Expr, CompileError> {
        self.expect(&Tok::LParen)?;
        let e = self.expr()?;
        self.expect(&Tok::RParen)?;
        Ok(Expr::Inv(Box::new(e)))
    }
}
