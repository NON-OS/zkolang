/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Unroll a bounded loop.

use super::super::compiler::{Compiler, MAX_OPS, MAX_UNROLL};
use crate::lang::parse::Stmt;
use crate::lang::CompileError;
use alloc::string::String;

impl Compiler {
    /// Unroll the loop over `[lo, hi)`: for each value, bind the loop variable as a
    /// compile-time constant, lower the body inline, then pop the binding. Bodies are
    /// flat, so a binding a body makes persists, which is what an accumulator needs.
    pub(crate) fn for_loop(
        &mut self,
        var: &str,
        lo: u64,
        hi: u64,
        body: &[Stmt],
    ) -> Result<(), CompileError> {
        if hi.saturating_sub(lo) > MAX_UNROLL {
            return Err(CompileError::LoopTooLarge);
        }
        let mut v = lo;
        while v < hi {
            self.loop_consts.push((String::from(var), v));
            for s in body {
                self.stmt(s)?;
            }
            self.loop_consts.pop();
            // Checked each iteration so a nested loop trips the bound partway through
            // its expansion, before the emitted vector can grow without limit.
            if self.ops.len() > MAX_OPS {
                return Err(CompileError::ProgramTooLong);
            }
            v += 1;
        }
        Ok(())
    }
}
