/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Assert two expressions differ.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Prove inequality by inverting the difference, which has a value only when the
    /// difference is nonzero, so a zero difference makes the trace unprovable. The
    /// inverse result is discarded; its only job is to constrain.
    pub(crate) fn assert_ne(&mut self, l: &Expr, r: &Expr) -> Result<(), CompileError> {
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        self.release(&a);
        self.release(&b);
        let diff = self.alloc()?;
        self.ops.push(Op::Sub {
            d: diff,
            a: a.reg,
            b: b.reg,
        });
        self.free.push(diff);
        let recip = self.alloc()?;
        self.ops.push(Op::Inv { d: recip, a: diff });
        self.free.push(recip);
        Ok(())
    }
}
