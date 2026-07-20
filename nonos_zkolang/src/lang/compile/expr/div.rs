/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Field division.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Division is sugar with no opcode of its own: `a / b` is `a * b^{-1}`. Because
    /// inverting zero has no valid trace, dividing by zero is unprovable rather than a
    /// wrong answer.
    pub(crate) fn div(&mut self, l: &Expr, r: &Expr) -> Result<Val, CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&b);
        let recip = self.alloc()?;
        self.ops.push(Op::Inv { d: recip, a: b.reg });
        self.release(&a);
        self.free.push(recip);
        let d = self.alloc()?;
        self.ops.push(Op::Mul {
            d,
            a: a.reg,
            b: recip,
        });
        Ok(Val { reg: d, temp: true })
    }
}
