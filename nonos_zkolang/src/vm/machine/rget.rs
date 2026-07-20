/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bounds-checked register read.

use nonos_stark::field::Fp;

use super::Vm;
use crate::vm::ProveError;

impl Vm {
    /// Read a register, bounds-checked. An out-of-range index is a typed error rather
    /// than a panic, so a malformed program cannot crash the executor.
    pub(crate) fn rget(&self, idx: u8) -> Result<Fp, ProveError> {
        self.regs
            .get(idx as usize)
            .copied()
            .ok_or(ProveError::BadRegister(idx))
    }
}
