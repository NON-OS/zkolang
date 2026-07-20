/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind a name to the next public input.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::CompileError;
use alloc::string::String;

impl Compiler {
    /// Allocate a register, read the next public input into it, and bind the name.
    pub(crate) fn input(&mut self, name: &str) -> Result<(), CompileError> {
        let d = self.alloc()?;
        let idx = self.next_public;
        self.next_public += 1;
        self.ops.push(Op::Inp { d, idx });
        self.syms.push((String::from(name), d));
        Ok(())
    }
}
