/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind an array literal to a name.

use alloc::vec::Vec;

use super::super::compiler::Compiler;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile each element to a register the array then owns, and bind the vector. A
    /// temporary element is not released, since the array keeps it; an element that
    /// aliases a binding is sound because a binding never mutates a register in place.
    /// A same-named scalar no longer applies once the name is an array.
    pub(crate) fn let_array(&mut self, name: &str, elems: &[Expr]) -> Result<(), CompileError> {
        let mut regs = Vec::with_capacity(elems.len());
        for el in elems {
            regs.push(self.expr(el)?.reg);
        }
        if let Some(old) = self.take_scalar(name) {
            if !regs.contains(&old) && !self.reg_in_use(old) {
                self.free.push(old);
            }
        }
        self.bind_array(name, regs);
        Ok(())
    }
}
