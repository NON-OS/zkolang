/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Resolve an array element to the register that holds it.

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Resolve `name[index]` to the register holding that element. The index must
    /// fold to a constant in range. The register is owned by the array, so a read is
    /// not a temporary the caller may free.
    pub(crate) fn array_element(
        &self,
        name: &str,
        index: &Expr,
        at: usize,
    ) -> Result<u8, CompileError> {
        let regs = self
            .lookup_array(name)
            .ok_or(CompileError::NotIndexable { at })?;
        let i = self.const_eval(index)?;
        if i < 0 || i as usize >= regs.len() {
            return Err(CompileError::IndexOutOfBounds { at });
        }
        Ok(regs[i as usize])
    }
}
