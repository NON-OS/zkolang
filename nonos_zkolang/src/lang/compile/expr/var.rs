/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A variable reference.

use super::super::compiler::{Compiler, Val};
use crate::lang::CompileError;

impl Compiler {
    /**
     * Names resolve innermost first: a loop variable, a binding such as a parameter or a
     * block local, then a program constant. A loop variable and a constant are
     * compile-time values and materialize as immediates; a bare array name is a whole
     * vector, not a value, and an unbound name is unknown.
     */
    pub(crate) fn emit_var(&mut self, n: &str) -> Result<Val, CompileError> {
        if let Some(v) = self.loop_const(n) {
            return self.emit_num(v);
        }
        if let Some(reg) = self.lookup(n) {
            return Ok(Val { reg, temp: false });
        }
        if self.lookup_array(n).is_some() {
            return Err(CompileError::ArrayNotScalar);
        }
        if let Some(v) = self.scalar_const(n) {
            return self.emit_num(v);
        }
        Err(CompileError::UnknownVariable {
            name: alloc::string::String::from(n),
        })
    }
}
