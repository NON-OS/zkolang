/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Construct a fresh machine.

use nonos_stark::field::Fp;

use super::Vm;
use crate::isa::REGS;

impl Vm {
    /// A fresh machine with every register zeroed.
    pub fn new() -> Vm {
        Vm {
            regs: [Fp::ZERO; REGS],
        }
    }
}
