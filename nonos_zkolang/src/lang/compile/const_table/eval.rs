/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Fold a compile-time-constant index expression to an integer.

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Fold a compile-time-constant expression for a table index. Only static pieces
    /// are allowed: literals, loop variables, arithmetic over them, and a nested
    /// table read. A runtime binding is a `NonConstantIndex` error, since an index on
    /// a witness would break the straight-line shape. The value is carried as `i128`
    /// so a subtraction may dip negative before the bounds check in the resolve step.
    pub(crate) fn const_eval(&self, e: &Expr) -> Result<i128, CompileError> {
        match e {
            Expr::Num(v) => Ok(*v as i128),
            Expr::Var(n) => self
                .loop_const(n)
                .map(|v| v as i128)
                .ok_or(CompileError::NonConstantIndex),
            Expr::Add(l, r) => Ok(self.const_eval(l)? + self.const_eval(r)?),
            Expr::Sub(l, r) => Ok(self.const_eval(l)? - self.const_eval(r)?),
            Expr::Mul(l, r) => Ok(self.const_eval(l)? * self.const_eval(r)?),
            Expr::Neg(x) => Ok(-self.const_eval(x)?),
            Expr::Index(base, idx) => Ok(self.resolve_index(base, idx)? as i128),
            _ => Err(CompileError::NonConstantIndex),
        }
    }
}
