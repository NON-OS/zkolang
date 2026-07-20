/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Multiplication and division, left-associative.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// Multiply and divide, binding tighter than add and subtract. Operands are unary
    /// expressions, so a leading minus binds tighter still.
    pub(crate) fn product(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.unary()?;
        loop {
            match self.peek() {
                Some(Tok::Star) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
                }
                Some(Tok::Slash) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    lhs = Expr::Div(Box::new(lhs), Box::new(rhs));
                }
                _ => return Ok(lhs),
            }
        }
    }
}
