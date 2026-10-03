/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The expression entry point.

use super::nesting::EXPR_COST;
use super::Parser;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;

impl<'a> Parser<'a> {
    /// An expression, lowest precedence first: logical or binds loosest.
    pub(crate) fn expr(&mut self) -> Result<Expr, CompileError> {
        /*
         * Links of operator chains stay spent until the whole expression closes, so every
         * edge on a path through the tree it builds is counted once.
         */
        let base = self.depth;
        self.spend(EXPR_COST)?;
        let e = self.logic_or();
        self.depth = base;
        e
    }
}
