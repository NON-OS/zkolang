/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Resolve `table[index]` to the field value it names.

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// The base must be a named constant table, the index must fold to a constant in
    /// range, and the result is the entry itself. A name bound to a runtime value is
    /// a scalar, so indexing it is a type error; an undeclared name is unknown.
    pub(crate) fn resolve_index(&self, base: &Expr, index: &Expr) -> Result<u64, CompileError> {
        let name = match base {
            Expr::Var(n) => n.as_str(),
            _ => return Err(CompileError::NotIndexable),
        };
        let is_value = self.lookup(name).is_some()
            || self.loop_const(name).is_some()
            || self.scalar_const(name).is_some();
        let table = match self.const_table(name) {
            Some(t) => t,
            None if is_value => return Err(CompileError::NotIndexable),
            None => {
                return Err(CompileError::UnknownConst {
                    name: alloc::string::String::from(name),
                })
            }
        };
        let i = self.const_eval(index)?;
        if i < 0 || i as usize >= table.len() {
            return Err(CompileError::IndexOutOfBounds);
        }
        Ok(table[i as usize])
    }
}
