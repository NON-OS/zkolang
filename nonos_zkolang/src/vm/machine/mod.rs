/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The machine state: a fixed register file over the field, with the bounds-checked
//! reads and writes the run and step code go through.

mod new;
mod rget;
mod wset;

use nonos_stark::field::Fp;

use crate::isa::REGS;

/// The machine: a fixed register file over the field. Every value it holds is an
/// `Fp`, the same scalar the STARK commits, so a run and its proof share one field.
/// When `check` is false the machine evaluates without enforcing constraints, which
/// the driver uses to read the values a comparison decomposes before it can supply the
/// bits; the proving run always checks.
pub struct Vm {
    regs: [Fp; REGS],
    pub(crate) check: bool,
}

impl Default for Vm {
    fn default() -> Vm {
        Vm::new()
    }
}
