/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A block expression: local bindings and a result, in a nested scope.

use alloc::vec::Vec;

use super::super::compiler::{Compiler, Val};
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile the bindings in order, each into a scope entry the later bindings and the
    /// result can see, then compile the result. On the way out the block's own bindings
    /// leave the scope and the registers they alone held return to the pool, while the
    /// result and anything an outer name still aliases are kept. The alias check makes
    /// this sound: a register a live binding shares is never freed under the block.
    pub(crate) fn block_expr(
        &mut self,
        locals: &[(alloc::string::String, Expr)],
        result: &Expr,
    ) -> Result<Val, CompileError> {
        let mark = self.syms.len();
        for (name, value) in locals {
            let v = self.expr(value)?;
            self.syms.push((name.clone(), v.reg));
        }
        let out = self.expr(result)?;
        let held: Vec<u8> = self
            .syms
            .split_off(mark)
            .into_iter()
            .map(|(_, r)| r)
            .collect();
        for r in held {
            if r != out.reg && !self.reg_in_use(r) && !self.free.contains(&r) {
                self.free.push(r);
            }
        }
        // The result is a temporary the caller may free exactly when no binding still
        // holds its register, once the block's own bindings are gone.
        let temp = !self.reg_in_use(out.reg);
        Ok(Val { reg: out.reg, temp })
    }
}
