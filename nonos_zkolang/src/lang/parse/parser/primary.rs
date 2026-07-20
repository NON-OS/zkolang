/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! An atom followed by index suffixes.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// A primary is an atom followed by any number of `[index]` suffixes, so `RC[i]`
    /// parses left to right. Indexing binds tighter than the unary minus above it.
    pub(crate) fn primary(&mut self) -> Result<Expr, CompileError> {
        let mut base = self.atom()?;
        while matches!(self.peek(), Some(Tok::LBracket)) {
            self.pos += 1;
            let index = self.expr()?;
            self.expect(&Tok::RBracket)?;
            base = Expr::Index(Box::new(base), Box::new(index));
        }
        Ok(base)
    }
}
