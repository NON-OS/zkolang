/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Assert two expressions are equal.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Prove equality by asserting the difference is zero.
    pub(crate) fn assert_eq(&mut self, l: &Expr, r: &Expr) -> Result<(), CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&a);
        self.release(&b);
        let d = self.alloc()?;
        self.ops.push(Op::Sub {
            d,
            a: a.reg,
            b: b.reg,
        });
        self.ops.push(Op::Assert { a: d });
        self.free.push(d);
        Ok(())
    }
}
