/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The match expression. `match e { v0 => a, v1 => b, _ => d }` desugars to a nested
//! select, comparing the scrutinee to each value in turn and falling through to the
//! default arm. The default `_` is required and comes last, so a match is exhaustive.
//! Both arms of every select are evaluated, as everywhere in the language.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    pub(crate) fn match_expr(&mut self) -> Result<Expr, CompileError> {
        let scrut = self.expr()?;
        self.expect(&Tok::LBrace)?;
        let mut specific: Vec<(u64, Expr)> = Vec::new();
        let mut default: Option<Expr> = None;
        loop {
            let at = self.at();
            if matches!(self.peek(), Some(Tok::Ident(n)) if n == "_") {
                self.pos += 1;
                self.expect(&Tok::FatArrow)?;
                default = Some(self.expr()?);
            } else {
                let v = self.number()?;
                self.expect(&Tok::FatArrow)?;
                let body = self.expr()?;
                if default.is_some() {
                    return Err(CompileError::UnexpectedToken { at });
                }
                specific.push((v, body));
            }
            match self.peek() {
                Some(Tok::Comma) => {
                    self.pos += 1;
                    if matches!(self.peek(), Some(Tok::RBrace)) {
                        break;
                    }
                }
                Some(Tok::RBrace) => break,
                _ => return Err(CompileError::UnexpectedToken { at: self.at() }),
            }
        }
        self.expect(&Tok::RBrace)?;
        let mut acc = default.ok_or(CompileError::UnexpectedToken { at: self.at() })?;
        for (v, body) in specific.into_iter().rev() {
            let cond = Expr::Eq(Box::new(scrut.clone()), Box::new(Expr::Num(v)));
            acc = Expr::Sel(Box::new(cond), Box::new(body), Box::new(acc));
        }
        Ok(acc)
    }
}
