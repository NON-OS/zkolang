/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A two-operand arithmetic node.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile both operands, release any temporaries so the result can reuse their
    /// registers, allocate the result, and emit the op the maker builds.
    pub(crate) fn binary(
        &mut self,
        l: &Expr,
        r: &Expr,
        make: fn(u8, u8, u8) -> Op,
    ) -> Result<Val, CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&a);
        self.release(&b);
        let d = self.alloc()?;
        self.ops.push(make(d, a.reg, b.reg));
        Ok(Val { reg: d, temp: true })
    }
}
