/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind a name to the next public input.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::CompileError;

impl Compiler {
    /// Allocate a register, read the next public input into it, and bind the name.
    pub(crate) fn input(&mut self, name: &str) -> Result<(), CompileError> {
        let d = self.alloc()?;
        let idx = self.take_public()?;
        self.ops.push(Op::Inp { d, idx });
        self.bind_fresh(name, d);
        Ok(())
    }
}
