/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The equality level, the lowest precedence.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// An optional single comparison of two sums. It is not chainable, because a
    /// comparison yields a bit and comparing that to a third sum is rarely meant.
    pub(crate) fn equality(&mut self) -> Result<Expr, CompileError> {
        let lhs = self.sum()?;
        match self.peek() {
            Some(Tok::EqEq) => {
                self.pos += 1;
                let rhs = self.sum()?;
                Ok(Expr::Eq(Box::new(lhs), Box::new(rhs)))
            }
            Some(Tok::BangEq) => {
                self.pos += 1;
                let rhs = self.sum()?;
                Ok(Expr::Ne(Box::new(lhs), Box::new(rhs)))
            }
            _ => Ok(lhs),
        }
    }
}
