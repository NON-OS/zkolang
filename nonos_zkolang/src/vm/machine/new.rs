/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Construct a fresh machine.

use nonos_stark::field::Fp;

use super::Vm;
use crate::isa::REGS;

impl Vm {
    /// A fresh machine with every register zeroed, enforcing constraints.
    pub fn new() -> Vm {
        Vm {
            regs: [Fp::ZERO; REGS],
            check: true,
        }
    }

    /// A machine that evaluates without enforcing constraints, for reading the values a
    /// comparison decomposes before the advice bits exist.
    pub fn evaluator() -> Vm {
        Vm {
            regs: [Fp::ZERO; REGS],
            check: false,
        }
    }
}
