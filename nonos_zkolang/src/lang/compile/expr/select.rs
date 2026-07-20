/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The branchless select, shared by `sel` and `if`.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile the condition and both arms, release the temporaries, and emit one
    /// select. Both arms are always evaluated, which keeps the trace shape independent
    /// of the data.
    pub(crate) fn select(&mut self, cond: &Expr, l: &Expr, r: &Expr) -> Result<Val, CompileError> {
        let c = self.expr(cond)?;
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&c);
        self.release(&a);
        self.release(&b);
        let d = self.alloc()?;
        self.ops.push(Op::Sel {
            d,
            c: c.reg,
            a: a.reg,
            b: b.reg,
        });
        Ok(Val { reg: d, temp: true })
    }
}
