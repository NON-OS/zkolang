/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind a name to the next private input.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::CompileError;
use alloc::string::String;

impl Compiler {
    /// Allocate a register and read the next secret input into it. Secret indices
    /// follow the public prefix, so a private witness is a hidden suffix of the input
    /// vector rather than part of the public statement.
    pub(crate) fn secret(&mut self, name: &str) -> Result<(), CompileError> {
        let d = self.alloc()?;
        let idx = self.n_public + self.next_secret;
        self.next_secret += 1;
        self.ops.push(Op::Inp { d, idx });
        self.syms.push((String::from(name), d));
        Ok(())
    }
}
