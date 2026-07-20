/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A prefix minus or logical not.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// A prefix minus negates and a prefix `!` is logical not, desugared to `1 - x` and
    /// exact when the operand is a bit. Both are right-recursive so `- - a` and `!!a`
    /// nest, and both sit above the primaries so `-a * b` parses as `(-a) * b`.
    pub(crate) fn unary(&mut self) -> Result<Expr, CompileError> {
        if matches!(self.peek(), Some(Tok::Minus)) {
            self.pos += 1;
            let inner = self.unary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        if matches!(self.peek(), Some(Tok::Bang)) {
            self.pos += 1;
            let inner = self.unary()?;
            return Ok(Expr::Sub(Box::new(Expr::Num(1)), Box::new(inner)));
        }
        self.primary()
    }
}
