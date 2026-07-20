/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind a scalar value to a name.

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Note the register the name held before, compile the value, then rebind. If the
    /// name shadowed a binding or an array whose registers no live name still holds,
    /// reclaim them. The alias check keeps this sound: a shared register is never
    /// freed under one name.
    pub(crate) fn let_scalar(&mut self, name: &str, e: &Expr) -> Result<(), CompileError> {
        let old = self.lookup(name);
        let v = self.expr(e)?;
        if let Some(old_array) = self.take_array(name) {
            for r in old_array {
                if r != v.reg && !self.reg_in_use(r) {
                    self.free.push(r);
                }
            }
        }
        self.rebind(name, v.reg);
        if let Some(old_reg) = old {
            if old_reg != v.reg && !self.reg_in_use(old_reg) {
                self.free.push(old_reg);
            }
        }
        Ok(())
    }
}
