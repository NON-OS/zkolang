/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bounds-checked register write.

use nonos_stark::field::Fp;

use super::Vm;
use crate::vm::ProveError;

impl Vm {
    /// Write a register, bounds-checked.
    pub(crate) fn wset(&mut self, idx: u8, v: Fp) -> Result<(), ProveError> {
        match self.regs.get_mut(idx as usize) {
            Some(slot) => {
                *slot = v;
                Ok(())
            }
            None => Err(ProveError::BadRegister(idx)),
        }
    }
}
