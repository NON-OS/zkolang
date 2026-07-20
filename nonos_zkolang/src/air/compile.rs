/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Compiling a program's public data flow into the AIR. This walks the program to
//! the first halt, records the per-row wiring, and binds each public input and
//! output to the row that reads or exposes it. The verifier reconstructs exactly
//! this from the same public program and public values.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::error::BuildError;
use super::layout::{A, IMM};
use super::step_air::StepAir;
use super::wiring::WireRow;
use crate::isa::{Op, Program};

impl StepAir {
    /// Build the AIR with only the program's wiring, no public value bindings. The
    /// periodic columns depend on the wiring alone, so this is all the verifier
    /// key's periodic root needs; it never sees the public inputs or outputs. The
    /// wiring and padding are identical to `compile`, so the periodic columns match
    /// exactly what a real proof commits.
    pub fn for_key(program: &Program, log_t: u32) -> Result<StepAir, BuildError> {
        let t = 1usize << log_t;
        let mut wiring: Vec<WireRow> = Vec::new();
        let mut halted = false;
        for op in program.iter() {
            wiring.push(WireRow::of(op));
            if matches!(op, Op::Halt) {
                halted = true;
                break;
            }
        }
        if !halted {
            return Err(BuildError::NoHalt);
        }
        if wiring.len() > t {
            return Err(BuildError::TooLong {
                rows: wiring.len(),
                cap: t,
            });
        }
        while wiring.len() < t {
            wiring.push(WireRow::EMPTY);
        }
        Ok(StepAir {
            log_t,
            wiring,
            public_bindings: Vec::new(),
        })
    }

    /// Compile a program's public data flow into the AIR and bind the public
    /// inputs and outputs. The wiring runs up to and including the first halt and
    /// is padded to the power-of-two length, matching how the VM stops at halt and
    /// `build_trace` pads.
    pub fn compile(
        program: &Program,
        log_t: u32,
        public_inputs: &[Fp],
        public_outputs: &[Fp],
    ) -> Result<StepAir, BuildError> {
        let t = 1usize << log_t;
        let mut wiring: Vec<WireRow> = Vec::new();
        let mut public_bindings: Vec<(usize, usize, Fp)> = Vec::new();
        let mut halted = false;
        for (row, op) in program.iter().enumerate() {
            wiring.push(WireRow::of(op));
            match *op {
                Op::Inp { idx, .. } => {
                    // A public input, whose index falls in the public prefix, is
                    // pinned to its committed value. A secret input has no value
                    // in `public_inputs` and is left unbound: a private witness
                    // that never enters the public statement.
                    if let Some(&v) = public_inputs.get(idx as usize) {
                        public_bindings.push((IMM, row, v));
                    }
                }
                Op::Out { idx, .. } => {
                    let v = public_outputs
                        .get(idx as usize)
                        .copied()
                        .ok_or(BuildError::MissingPublicOutput { idx })?;
                    public_bindings.push((A, row, v));
                }
                Op::Halt => {
                    halted = true;
                    break;
                }
                _ => {}
            }
        }
        if !halted {
            return Err(BuildError::NoHalt);
        }
        if wiring.len() > t {
            return Err(BuildError::TooLong {
                rows: wiring.len(),
                cap: t,
            });
        }
        while wiring.len() < t {
            wiring.push(WireRow::EMPTY);
        }
        Ok(StepAir {
            log_t,
            wiring,
            public_bindings,
        })
    }
}
