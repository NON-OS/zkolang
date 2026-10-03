/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A block expression: local bindings and a result, in a nested scope.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::compiler::{Compiler, Val};
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// A block in scalar position: open its locals, compile the result to one value, then
    /// close the scope. The result is a temporary the caller may free exactly when no binding
    /// still holds its register, once the block's own bindings are gone.
    pub(crate) fn block_expr(
        &mut self,
        locals: &[(Vec<String>, Expr)],
        result: &Expr,
    ) -> Result<Val, CompileError> {
        let mark = self.open_block(locals)?;
        let out = self.expr(result)?;
        self.close_block(mark, &[out.reg]);
        let temp = !self.reg_in_use(out.reg);
        Ok(Val { reg: out.reg, temp })
    }
}
