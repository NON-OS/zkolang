/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Destructure a tuple value into several names.

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile the right side in tuple mode, then bind each name to its value's register. The
    /// name count must match the arity the value produced. Each binding reclaims a register a
    /// name it shadows no longer holds, under the same alias check a scalar binding uses, so a
    /// shared register is never freed under one name.
    pub(crate) fn let_tuple(
        &mut self,
        names: &[alloc::string::String],
        e: &Expr,
    ) -> Result<(), CompileError> {
        let vals = self.expr_tuple(e)?;
        if names.len() != vals.len() {
            return Err(CompileError::TupleArity {
                names: names.len(),
                values: vals.len(),
            });
        }
        for (name, v) in names.iter().zip(&vals) {
            let old = self.lookup(name);
            if let Some(old_array) = self.take_array(name) {
                for r in old_array {
                    if r != v.reg && !self.reg_in_use(r) && !self.free.contains(&r) {
                        self.free.push(r);
                    }
                }
            }
            self.rebind(name, v.reg);
            if let Some(old_reg) = old {
                if old_reg != v.reg && !self.reg_in_use(old_reg) && !self.free.contains(&old_reg) {
                    self.free.push(old_reg);
                }
            }
        }
        Ok(())
    }
}
