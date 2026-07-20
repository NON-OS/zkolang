/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A variable reference.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::CompileError;
use nonos_stark::field::Fp;

impl Compiler {
    /// A loop variable is a compile-time constant and materializes as an immediate.
    /// Otherwise the name resolves to a binding; a bare array name is a whole vector,
    /// not a value, and an unbound name is unknown.
    pub(crate) fn emit_var(&mut self, n: &str) -> Result<Val, CompileError> {
        if let Some(v) = self.loop_const(n) {
            let d = self.alloc()?;
            self.ops.push(Op::Imm {
                d,
                v: Fp::from_u64(v),
            });
            return Ok(Val { reg: d, temp: true });
        }
        if let Some(reg) = self.lookup(n) {
            return Ok(Val { reg, temp: false });
        }
        if self.lookup_array(n).is_some() {
            return Err(CompileError::ArrayNotScalar);
        }
        Err(CompileError::UnknownVariable)
    }
}
