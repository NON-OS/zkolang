/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The expression entry point.

use super::Parser;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// An expression, lowest precedence first.
    pub(crate) fn expr(&mut self) -> Result<Expr, CompileError> {
        self.equality()
    }
}
