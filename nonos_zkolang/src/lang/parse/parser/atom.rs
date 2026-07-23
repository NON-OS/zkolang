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
        let at = self.at();
        match self.bump() {
            Some(Tok::Num(v)) => Ok(Expr::Num(*v)),
            Some(Tok::Ident(n)) => {
                let name = n.clone();
                self.ident_expr(name)
            }
            Some(Tok::LParen) => {
                let e = self.expr()?;
                if matches!(self.peek(), Some(Tok::Comma)) {
                    // More than one value between the parentheses is a tuple, the shape a
                    // function returns when it returns several things.
                    let mut elems = alloc::vec![e];
                    while matches!(self.peek(), Some(Tok::Comma)) {
                        self.pos += 1;
                        elems.push(self.expr()?);
                    }
                    self.expect(&Tok::RParen)?;
                    Ok(Expr::Tuple(elems))
                } else {
                    self.expect(&Tok::RParen)?;
                    Ok(e)
                }
            }
            Some(Tok::LBracket) => self.array_expr(),
            Some(Tok::LBrace) => self.block_body(),
            Some(Tok::Inv) => self.inv_expr(),
            Some(Tok::Sel) => self.sel_expr(),
            Some(Tok::If) => self.if_expr(),
            Some(Tok::Match) => self.match_expr(),
            Some(_) => Err(CompileError::UnexpectedToken { at }),
            None => Err(CompileError::UnexpectedEof { at }),
        }
    }
}
