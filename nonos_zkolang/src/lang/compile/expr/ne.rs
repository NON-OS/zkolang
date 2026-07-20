/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Not-equal.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;
use nonos_stark::field::Fp;

impl Compiler {
    /// Not-equal is the complement of the equality bit: `(a != b) = 1 - (a == b)`,
    /// which flips a clean zero-or-one bit to its opposite.
    pub(crate) fn ne(&mut self, l: &Expr, r: &Expr) -> Result<Val, CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&a);
        self.release(&b);
        let bit = self.alloc()?;
        self.ops.push(Op::Eq {
            d: bit,
            a: a.reg,
            b: b.reg,
        });
        let one = self.alloc()?;
        self.ops.push(Op::Imm { d: one, v: Fp::ONE });
        self.free.push(bit);
        self.free.push(one);
        let d = self.alloc()?;
        self.ops.push(Op::Sub { d, a: one, b: bit });
        Ok(Val { reg: d, temp: true })
    }
}
