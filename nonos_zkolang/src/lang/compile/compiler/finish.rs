/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Finish the program.

use alloc::vec::Vec;

use super::state::Compiler;
use crate::isa::Op;

impl Compiler {
    /// End the program with a halt and hand back the instruction list.
    pub(crate) fn finish(mut self) -> Vec<Op> {
        self.ops.push(Op::Halt);
        self.ops
    }
}
