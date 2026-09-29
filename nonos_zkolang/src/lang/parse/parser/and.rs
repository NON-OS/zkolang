/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Logical and, binding tighter than or and looser than comparison. It is desugared to
//! `a * b`, exact when both operands are bits. Left associative.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    pub(crate) fn logic_and(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.equality()?;
        let mut links = 0;
        while matches!(self.peek(), Some(Tok::AmpAmp)) {
            self.pos += 1;
            self.enter()?;
            links += 1;
            let rhs = self.equality()?;
            lhs = Expr::Mul(Box::new(lhs), Box::new(rhs));
        }
        self.leave(links);
        Ok(lhs)
    }
}
