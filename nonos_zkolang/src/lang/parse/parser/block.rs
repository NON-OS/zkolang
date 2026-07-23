/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A block: local bindings and a result.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// The body of a block, with the opening brace already consumed: zero or more
    /// `let name = expr;` bindings followed by the result, written either as `return
    /// expr;` or as a trailing expression. The result is required, so a block always
    /// has a value; the bindings scope to the block and to each other in order.
    pub(crate) fn block_body(&mut self) -> Result<Expr, CompileError> {
        let mut locals = Vec::new();
        loop {
            match self.peek() {
                Some(Tok::Let) => {
                    self.pos += 1;
                    let name = self.ident()?;
                    self.expect(&Tok::Assign)?;
                    let value = self.expr()?;
                    self.expect(&Tok::Semi)?;
                    locals.push((name, value));
                }
                Some(Tok::Return) => {
                    self.pos += 1;
                    let result = self.expr()?;
                    self.expect(&Tok::Semi)?;
                    self.expect(&Tok::RBrace)?;
                    return Ok(Expr::Block(locals, Box::new(result)));
                }
                Some(Tok::RBrace) => return Err(CompileError::UnexpectedToken { at: self.at() }),
                None => return Err(CompileError::UnexpectedEof { at: self.at() }),
                _ => {
                    let result = self.expr()?;
                    if matches!(self.peek(), Some(Tok::Semi)) {
                        self.pos += 1;
                    }
                    self.expect(&Tok::RBrace)?;
                    return Ok(Expr::Block(locals, Box::new(result)));
                }
            }
        }
    }
}
