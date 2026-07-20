/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Negation.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;
use nonos_stark::field::Fp;

impl Compiler {
    /// Negation is subtraction from zero: `-x = 0 - x`, so no dedicated opcode.
    pub(crate) fn neg(&mut self, x: &Expr) -> Result<Val, CompileError> {
        let v = self.expr(x)?;
        let zero = self.alloc()?;
        self.ops.push(Op::Imm {
            d: zero,
            v: Fp::ZERO,
        });
        self.release(&v);
        self.free.push(zero);
        let d = self.alloc()?;
        self.ops.push(Op::Sub {
            d,
            a: zero,
            b: v.reg,
        });
        Ok(Val { reg: d, temp: true })
    }
}
