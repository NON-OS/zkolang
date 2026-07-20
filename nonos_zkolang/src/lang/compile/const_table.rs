/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Constant tables and the compile-time index that reads them. A table is a fixed
//! list of field values named once and read by a static index; because both the
//! table and the index are known at compile time, a read resolves to a single value
//! here and the table never reaches the trace. This is what turns a hash's hundreds
//! of round constants into `RC[r * width + i]` instead of hundreds of literals.

use super::super::parse::Expr;
use super::super::CompileError;
use super::compiler::Compiler;

impl Compiler {
    /// The values of a constant table by name, in declaration order.
    pub(super) fn const_table(&self, name: &str) -> Option<&[u64]> {
        self.consts
            .iter()
            .find(|c| c.name.as_str() == name)
            .map(|c| c.values.as_slice())
    }

    /// Fold a compile-time-constant expression to its integer value, for a table
    /// index. Only the genuinely static pieces are allowed: literals, loop variables,
    /// the arithmetic that combines them, and a nested table read. A reference to a
    /// runtime binding is a `NonConstantIndex` error, because an index that depended
    /// on a witness would break the straight-line shape the AIR relies on. The value
    /// is carried as `i128` so an intermediate subtraction may dip negative before
    /// the bounds check in `resolve_index`.
    pub(super) fn const_eval(&self, e: &Expr) -> Result<i128, CompileError> {
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

    /// Resolve `table[index]` to the field value it names: the base must be a named
    /// constant table, the index must fold to a constant in range, and the result is
    /// the entry itself. This is the one place a table read becomes a value.
    pub(super) fn resolve_index(&self, base: &Expr, index: &Expr) -> Result<u64, CompileError> {
        let name = match base {
            Expr::Var(n) => n.as_str(),
            _ => return Err(CompileError::NotIndexable),
        };
        let table = match self.const_table(name) {
            Some(t) => t,
            // A name that resolves to a runtime binding or loop variable is a scalar,
            // so indexing it is a type error; a name that resolves to nothing is an
            // undeclared table. The two read as different mistakes.
            None if self.lookup(name).is_some() || self.loop_const(name).is_some() => {
                return Err(CompileError::NotIndexable);
            }
            None => return Err(CompileError::UnknownConst),
        };
        let i = self.const_eval(index)?;
        if i < 0 || i as usize >= table.len() {
            return Err(CompileError::IndexOutOfBounds);
        }
        Ok(table[i as usize])
    }
}
