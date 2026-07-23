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
    /// Bind a block's locals in order, each visible to the ones after it. A local names one
    /// value, or several with `let (x, y) = e;`, in which case the right side is compiled in
    /// tuple mode and its arity must match the names. Returns the scope mark to restore at the
    /// end of the block.
    pub(crate) fn open_block(
        &mut self,
        locals: &[(Vec<String>, Expr)],
    ) -> Result<usize, CompileError> {
        let mark = self.syms.len();
        for (names, value) in locals {
            if names.len() == 1 {
                let v = self.expr(value)?;
                self.syms.push((names[0].clone(), v.reg));
            } else {
                let vals = self.expr_tuple(value)?;
                if vals.len() != names.len() {
                    return Err(CompileError::TupleArity {
                        names: names.len(),
                        values: vals.len(),
                    });
                }
                for (n, v) in names.iter().zip(&vals) {
                    self.syms.push((n.clone(), v.reg));
                }
            }
        }
        Ok(mark)
    }

    /// Drop the block's locals back to `mark` and return the registers they alone held to the
    /// pool, keeping the result registers and anything an outer name still aliases. The alias
    /// check makes this sound: a register a live binding shares is never freed under the block.
    pub(crate) fn close_block(&mut self, mark: usize, result_regs: &[u8]) {
        let held: Vec<u8> = self
            .syms
            .split_off(mark)
            .into_iter()
            .map(|(_, r)| r)
            .collect();
        for r in held {
            if !result_regs.contains(&r) && !self.reg_in_use(r) && !self.free.contains(&r) {
                self.free.push(r);
            }
        }
    }

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
