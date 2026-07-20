/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Compile a program's public data flow into the AIR, binding public inputs and
//! outputs. The wiring runs up to and including the first halt and is padded to the
//! power-of-two length, matching how the VM stops and `build_trace` pads.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::bind::bind_op;
use super::error::BuildError;
use super::pad::pad_and_build;
use super::step_air::StepAir;
use super::wiring::WireRow;
use crate::isa::Program;

impl StepAir {
    /// Compile and bind the public inputs and outputs to the rows that read or
    /// expose them.
    pub fn compile(
        program: &Program,
        log_t: u32,
        public_inputs: &[Fp],
        public_outputs: &[Fp],
    ) -> Result<StepAir, BuildError> {
        let mut wiring: Vec<WireRow> = Vec::new();
        let mut binds: Vec<(usize, usize, Fp)> = Vec::new();
        let mut halted = false;
        for (row, op) in program.iter().enumerate() {
            wiring.push(WireRow::of(op));
            if bind_op(row, *op, public_inputs, public_outputs, &mut binds)? {
                halted = true;
                break;
            }
        }
        pad_and_build(wiring, halted, log_t, binds)
    }
}
