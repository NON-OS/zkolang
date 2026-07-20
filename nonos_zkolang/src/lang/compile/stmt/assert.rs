/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Lower an assertion.

use super::super::compiler::Compiler;
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Writing the comparison out reads naturally: `assert a == b` proves equality
    /// and `assert a != b` proves inequality, each through its own lowering. Any other
    /// expression is asserted to be zero directly.
    pub(crate) fn assert(&mut self, e: &Expr) -> Result<(), CompileError> {
        match e {
            Expr::Eq(l, r) => self.assert_eq(l, r),
            Expr::Ne(l, r) => self.assert_ne(l, r),
            _ => {
                let v = self.expr(e)?;
                self.ops.push(Op::Assert { a: v.reg });
                self.release(&v);
                Ok(())
            }
        }
    }
}
