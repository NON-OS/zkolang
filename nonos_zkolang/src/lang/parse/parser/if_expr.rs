/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The conditional expression.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// `if c { a } else { b }`: the same select in a familiar shape. Both arms are
    /// single expressions, because the lowering to select evaluates both.
    pub(crate) fn if_expr(&mut self) -> Result<Expr, CompileError> {
        let cond = self.expr()?;
        self.expect(&Tok::LBrace)?;
        let a = self.expr()?;
        self.expect(&Tok::RBrace)?;
        self.expect(&Tok::Else)?;
        self.expect(&Tok::LBrace)?;
        let b = self.expr()?;
        self.expect(&Tok::RBrace)?;
        Ok(Expr::If(Box::new(cond), Box::new(a), Box::new(b)))
    }
}
