/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Fold a compile-time-constant index expression to its field value.

use nonos_stark::field::Fp;

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Fold a compile-time-constant expression for a table index. Only static pieces
    /// are allowed: literals, loop variables, arithmetic over them, and a nested
    /// table read. A runtime binding is a `NonConstantIndex` error, since an index on
    /// a witness would break the straight-line shape. The arithmetic is the field's, the
    /// same the optimizer folds the index with, so an optimized and an unoptimized build
    /// read the same entry and no intermediate can overflow; the bounds check is on the
    /// canonical value.
    pub(crate) fn const_eval(&self, e: &Expr) -> Result<Fp, CompileError> {
        match e {
            Expr::Num(v) => Ok(Fp::from_u64(*v)),
            Expr::Var(n) => self
                .loop_const(n)
                .map(Fp::from_u64)
                .ok_or(CompileError::NonConstantIndex),
            Expr::Add(l, r) => Ok(self.const_eval(l)? + self.const_eval(r)?),
            Expr::Sub(l, r) => Ok(self.const_eval(l)? - self.const_eval(r)?),
            Expr::Mul(l, r) => Ok(self.const_eval(l)? * self.const_eval(r)?),
            Expr::Neg(x) => Ok(Fp::ZERO - self.const_eval(x)?),
            Expr::Index(base, idx, at) => Ok(Fp::from_u64(self.resolve_index(base, idx, *at)?)),
            _ => Err(CompileError::NonConstantIndex),
        }
    }

    /// The position a folded index names in a run of `len` elements, if it is in bounds.
    pub(crate) fn const_position(
        &self,
        e: &Expr,
        len: usize,
    ) -> Result<Option<usize>, CompileError> {
        let v = self.const_eval(e)?.value();
        Ok(usize::try_from(v).ok().filter(|&i| i < len))
    }
}
