/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Prove and verify an already-compiled program.

use nonos_stark::field::Fp;

use super::{Report, RunError};
use crate::isa::Op;

/// Run a VM program on `inputs` (all treated as public), prove it, and verify the
/// proof. Returns the report including the public outputs.
pub fn prove_program(program: &[Op], inputs: &[Fp]) -> Result<Report, RunError> {
    super::pipeline::run_and_prove(program, inputs, inputs.len())
}
