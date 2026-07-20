/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Logical or, the loosest binding operator. It is desugared to the field encoding
//! `a + b - a * b`, exact when both operands are bits, as a comparison or the other
//! boolean operators produce. Left associative.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    pub(crate) fn logic_or(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.logic_and()?;
        while matches!(self.peek(), Some(Tok::PipePipe)) {
            self.pos += 1;
            let rhs = self.logic_and()?;
            let sum = Expr::Add(Box::new(lhs.clone()), Box::new(rhs.clone()));
            let prod = Expr::Mul(Box::new(lhs), Box::new(rhs));
            lhs = Expr::Sub(Box::new(sum), Box::new(prod));
        }
        Ok(lhs)
    }
}
