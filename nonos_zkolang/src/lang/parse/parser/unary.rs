/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A prefix minus.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// A prefix minus negates, right-recursive so `- - a` double-negates, above the
    /// primaries so `-a * b` parses as `(-a) * b`.
    pub(crate) fn unary(&mut self) -> Result<Expr, CompileError> {
        if matches!(self.peek(), Some(Tok::Minus)) {
            self.pos += 1;
            let inner = self.unary()?;
            return Ok(Expr::Neg(Box::new(inner)));
        }
        self.primary()
    }
}
