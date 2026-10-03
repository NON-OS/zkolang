/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The work budget. Lowering stops with `ProgramTooLong` once it has emitted more
 * instructions, inlined more calls or unrolled more loop iterations than any provable
 * program needs, so no input can keep it busy for hours or exhaust memory.
 */

use super::limits::{MAX_INLINES, MAX_ITERATIONS, MAX_OPS};
use super::state::Compiler;
use crate::lang::CompileError;

impl Compiler {
    /** Refuse once the emitted instructions pass the cap. */
    pub(crate) fn check_emitted(&self) -> Result<(), CompileError> {
        if self.ops.len() > MAX_OPS {
            return Err(CompileError::ProgramTooLong);
        }
        Ok(())
    }

    /** Spend one inlined call. */
    pub(crate) fn spend_inline(&mut self) -> Result<(), CompileError> {
        self.inlines += 1;
        if self.inlines > MAX_INLINES {
            return Err(CompileError::ProgramTooLong);
        }
        self.check_emitted()
    }

    /** Spend one unrolled loop iteration. */
    pub(crate) fn spend_iteration(&mut self) -> Result<(), CompileError> {
        self.iterations += 1;
        if self.iterations > MAX_ITERATIONS {
            return Err(CompileError::ProgramTooLong);
        }
        self.check_emitted()
    }
}
