/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The branchless-select builtin.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// `sel(c, a, b)`: the branchless conditional.
    pub(crate) fn sel_expr(&mut self) -> Result<Expr, CompileError> {
        self.expect(&Tok::LParen)?;
        let cond = self.expr()?;
        self.expect(&Tok::Comma)?;
        let a = self.expr()?;
        self.expect(&Tok::Comma)?;
        let b = self.expr()?;
        self.expect(&Tok::RParen)?;
        Ok(Expr::Sel(Box::new(cond), Box::new(a), Box::new(b)))
    }
}
