/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A numeric literal.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::CompileError;
use nonos_stark::field::Fp;

impl Compiler {
    /// Load a literal into a fresh register.
    pub(crate) fn emit_num(&mut self, v: u64) -> Result<Val, CompileError> {
        let d = self.alloc()?;
        self.ops.push(Op::Imm {
            d,
            v: Fp::from_u64(v),
        });
        Ok(Val { reg: d, temp: true })
    }
}
