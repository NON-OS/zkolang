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
        for (i, (name, v)) in names.iter().zip(&vals).enumerate() {
            /*
             * A register this binding would free may still be carried by a later slot of
             * the same destructure, `(y, x)` rebinding `x` and then `y`: it stays until that
             * slot is bound.
             */
            let later = |r: u8| {
                names[i + 1..]
                    .iter()
                    .zip(&vals[i + 1..])
                    .any(|(n, w)| n != "_" && w.reg == r)
            };
            if name == "_" {
                /* A wildcard binds nothing and frees its register when nothing holds it. */
                if !later(v.reg) && !self.reg_in_use(v.reg) {
                    self.free_reg(v.reg);
                }
                continue;
            }
            let old = self.lookup(name);
            let old_array = self.take_array(name).unwrap_or_default();
            self.rebind(name, v.reg);
            for r in old.into_iter().chain(old_array) {
                if r != v.reg && !later(r) && !self.reg_in_use(r) {
                    self.free_reg(r);
                }
            }
        }
        Ok(())
    }
}
