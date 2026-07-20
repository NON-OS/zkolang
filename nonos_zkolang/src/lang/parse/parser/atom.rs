/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A primary atom, dispatched on its leading token.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// A literal, a variable or call, a parenthesized expression, an array literal, or
    /// one of the builtin and conditional forms.
    pub(crate) fn atom(&mut self) -> Result<Expr, CompileError> {
        match self.bump() {
            Some(Tok::Num(v)) => Ok(Expr::Num(*v)),
            Some(Tok::Ident(n)) => {
                let name = n.clone();
                self.ident_expr(name)
            }
            Some(Tok::LParen) => {
                let e = self.expr()?;
                self.expect(&Tok::RParen)?;
                Ok(e)
            }
            Some(Tok::LBracket) => self.array_expr(),
            Some(Tok::Inv) => self.inv_expr(),
            Some(Tok::Sel) => self.sel_expr(),
            Some(Tok::If) => self.if_expr(),
            Some(_) => Err(CompileError::UnexpectedToken),
            None => Err(CompileError::UnexpectedEof),
        }
    }
}
