/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The machine state: a fixed register file over the field, and the bounds-checked
//! reads and writes the run and step code go through. Keeping the register access
//! next to the state that owns it is what lets the register array stay private.

use nonos_stark::field::Fp;

use super::ProveError;
use crate::isa::REGS;

/// The machine: a fixed register file over the field. Every value it holds is an
/// `Fp`, the same scalar the STARK commits, so a run and its proof share one field.
pub struct Vm {
    regs: [Fp; REGS],
}

impl Default for Vm {
    fn default() -> Vm {
        Vm::new()
    }
}

impl Vm {
    /// A fresh machine with every register zeroed.
    pub fn new() -> Vm {
        Vm {
            regs: [Fp::ZERO; REGS],
        }
    }

    /// Read a register, bounds-checked. An out-of-range index is a typed error
    /// rather than a panic, so a malformed program cannot crash the executor.
    pub(super) fn rget(&self, idx: u8) -> Result<Fp, ProveError> {
        self.regs
            .get(idx as usize)
            .copied()
            .ok_or(ProveError::BadRegister(idx))
    }

    /// Write a register, bounds-checked.
    pub(super) fn wset(&mut self, idx: u8, v: Fp) -> Result<(), ProveError> {
        match self.regs.get_mut(idx as usize) {
            Some(slot) => {
                *slot = v;
                Ok(())
            }
            None => Err(ProveError::BadRegister(idx)),
        }
    }
}
