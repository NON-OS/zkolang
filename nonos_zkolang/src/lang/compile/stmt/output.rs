/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Expose an expression as the next public output.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile the expression, emit it as the next public output, and release the
    /// value, since the output vector now carries it.
    pub(crate) fn output(&mut self, e: &Expr) -> Result<(), CompileError> {
        let v = self.expr(e)?;
        let idx = self.next_output;
        self.next_output += 1;
        self.ops.push(Op::Out { a: v.reg, idx });
        self.release(&v);
        Ok(())
    }
}
