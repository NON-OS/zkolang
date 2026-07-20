/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! An index into an array or a constant table.

use super::super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;
use nonos_stark::field::Fp;

impl Compiler {
    /// Resolve against an array binding first, returning the register that element
    /// occupies, and otherwise against a constant table, where the value folds to one
    /// immediate. Both need a compile-time index.
    pub(crate) fn index(&mut self, base: &Expr, idx: &Expr) -> Result<Val, CompileError> {
        if let Expr::Var(name) = base {
            if self.lookup_array(name).is_some() {
                let reg = self.array_element(name, idx)?;
                return Ok(Val { reg, temp: false });
            }
        }
        let v = self.resolve_index(base, idx)?;
        let d = self.alloc()?;
        self.ops.push(Op::Imm {
            d,
            v: Fp::from_u64(v),
        });
        Ok(Val { reg: d, temp: true })
    }
}
