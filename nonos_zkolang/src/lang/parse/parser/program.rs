/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The top-level item loop.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Ast;
use crate::lang::CompileError;
use alloc::vec::Vec;

impl<'a> Parser<'a> {
    /// A program is a sequence of items: constant tables, functions, and statements.
    pub(crate) fn program(&mut self) -> Result<Ast, CompileError> {
        let mut consts = Vec::new();
        let mut fns = Vec::new();
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            match self.peek() {
                Some(Tok::Const) => consts.push(self.const_def()?),
                Some(Tok::Fn) => fns.push(self.fn_def()?),
                _ => stmts.push(self.stmt()?),
            }
        }
        Ok(Ast { consts, fns, stmts })
    }
}
