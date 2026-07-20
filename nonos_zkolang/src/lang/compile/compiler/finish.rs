/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Finish the program.

use super::state::Compiler;
use crate::isa::Op;
use crate::lang::compile::compiled::Compiled;

impl Compiler {
    /// End the program with a halt and hand back the program and its advice plan.
    pub(crate) fn finish(mut self) -> Compiled {
        self.ops.push(Op::Halt);
        Compiled {
            ops: self.ops,
            advice: self.advice,
            n_advice: self.next_advice,
        }
    }
}
