/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Which compile errors are about the machine's size rather than the program's meaning. */

use super::CompileError;

impl CompileError {
    /**
     * Whether the error is running out of registers, instructions or input indices. Such
     * an error depends on how the program is lowered, so an optimized program may avoid
     * it; every other error is about the program as written.
     */
    pub fn is_resource(&self) -> bool {
        matches!(
            self,
            CompileError::TooManyRegisters | CompileError::ProgramTooLong | CompileError::IoLimit
        )
    }
}
