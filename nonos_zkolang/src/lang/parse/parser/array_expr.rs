/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! An array literal.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// `[e0, e1, ...]`: the elements are full expressions, so an array can hold
    /// computed values, not only literals.
    pub(crate) fn array_expr(&mut self) -> Result<Expr, CompileError> {
        let mut elems = Vec::new();
        if !matches!(self.peek(), Some(Tok::RBracket)) {
            loop {
                elems.push(self.expr()?);
                if matches!(self.peek(), Some(Tok::Comma)) {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        self.expect(&Tok::RBracket)?;
        Ok(Expr::Array(elems))
    }
}
