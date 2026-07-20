/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Reserve a register.

use super::state::Compiler;
use crate::isa::REGS;
use crate::lang::CompileError;

impl Compiler {
    /// Reserve a register, reusing a freed one when the pool is non-empty and failing
    /// with `TooManyRegisters` when the file is exhausted.
    pub(crate) fn alloc(&mut self) -> Result<u8, CompileError> {
        if let Some(r) = self.free.pop() {
            return Ok(r);
        }
        if self.next as usize >= REGS {
            return Err(CompileError::TooManyRegisters);
        }
        let r = self.next;
        self.next += 1;
        Ok(r)
    }
}
