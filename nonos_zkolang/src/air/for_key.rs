/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Build the wiring-only AIR, with no public value bindings.

use alloc::vec::Vec;

use super::error::BuildError;
use super::pad::pad_and_build;
use super::step_air::StepAir;
use super::wiring::WireRow;
use crate::isa::{Op, Program};

impl StepAir {
    /// The AIR with only the program's wiring. The periodic columns depend on the
    /// wiring alone, so this is all the verifier key's periodic root needs; the
    /// wiring and padding match `compile` exactly, so the columns match a real proof.
    pub fn for_key(program: &Program, log_t: u32) -> Result<StepAir, BuildError> {
        let mut wiring: Vec<WireRow> = Vec::new();
        let mut halted = false;
        for op in program.iter() {
            wiring.push(WireRow::of(op));
            if matches!(op, Op::Halt) {
                halted = true;
                break;
            }
        }
        pad_and_build(wiring, halted, log_t, Vec::new())
    }
}
