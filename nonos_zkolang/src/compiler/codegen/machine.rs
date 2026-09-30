/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The result of code generation. */

use alloc::vec::Vec;

use crate::compiler::ssa::{Hint, V};
use crate::isa::Op;

/** What an instruction of the machine program does for the SSA program. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /** Computes the value, or applies the constraint or output, of the instruction. */
    Def(V),
    /** Reads the value again into a register. */
    Reload(V),
    /** One of the three instructions that constrain a copy of the value in the advice. */
    Spill(V),
    Halt,
}

/** A machine program, where each instruction comes from, and the advice it reads. */
#[derive(Clone, Debug, Default)]
pub struct Machine {
    pub ops: Vec<Op>,
    pub origins: Vec<Origin>,
    /** The hint of each advice slot, in order: the program's advice, then the copies. */
    pub advice: Vec<Hint>,
    /** How many input slots come before the advice. */
    pub n_inputs: usize,
}

/** Why code generation stopped. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodegenError {
    /** A gadget-level instruction was not expanded. */
    Unexpanded(V),
    /** A value was needed that no register holds and nothing can read again. */
    Lost(V),
    /** The inputs and advice need more slots than a 16-bit index names. */
    TooManySlots,
}
