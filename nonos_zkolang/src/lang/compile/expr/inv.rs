/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Field inverse.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// The field inverse; inverting zero makes the trace unprovable.
    pub(crate) fn inv(&mut self, x: &Expr) -> Result<Val, CompileError> {
        let a = self.expr(x)?;
        self.release(&a);
        let d = self.alloc()?;
        self.ops.push(Op::Inv { d, a: a.reg });
        Ok(Val { reg: d, temp: true })
    }
}
