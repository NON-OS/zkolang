/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Prove and verify an already-compiled program.

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::{Report, RunError};
use crate::isa::Op;

/// Run a VM program on `inputs` (all treated as public), prove it, and verify the
/// proof. Returns the report including the public outputs.
pub fn prove_program(program: &[Op], inputs: &[Fp]) -> Result<Report, RunError> {
    super::pipeline::run_and_prove(program, inputs, inputs.len())
}

/**
 * Run a VM program on `inputs`, the first `n_public` of which are the public statement and
 * the rest the private witness, prove it hiding the witness with the blinding expanded
 * from the private `seed`, and verify the proof. `TraceTooSmallToHide` for a trace too
 * short to carry the blinding.
 */
pub fn prove_program_hidden(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
    seed: &[Fp; RATE],
) -> Result<Report, RunError> {
    super::pipeline::run_and_prove_hidden(program, inputs, n_public.min(inputs.len()), seed)
}
