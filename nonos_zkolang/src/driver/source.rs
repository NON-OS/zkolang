/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The source-level entry points: compile first, then prove. These are the
//! functions a caller usually reaches for. Public inputs are supplied in
//! declaration order; a private witness feeds the run without entering the
//! public statement.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::advice::fill_advice;
use super::pipeline::run_and_prove;
use super::{Report, RunError};
use crate::lang::compile_source_full;

/// Compile zkolang source, then prove and verify it with the given public inputs.
pub fn prove_source_with_inputs(src: &str, inputs: &[u64]) -> Result<Report, RunError> {
    prove_source_with_witness(src, inputs, &[])
}

/// Compile zkolang source, then prove and verify it with public inputs and a
/// private witness. The `secret_inputs` feed the program's `secret` declarations;
/// they are used by the run but never enter the public statement, so a verifier
/// learns the outputs and the public inputs, not the witness.
pub fn prove_source_with_witness(
    src: &str,
    public_inputs: &[u64],
    secret_inputs: &[u64],
) -> Result<Report, RunError> {
    let compiled = compile_source_full(src).map_err(RunError::Compile)?;
    let mut inputs: Vec<Fp> = public_inputs.iter().map(|&v| Fp::from_u64(v)).collect();
    inputs.extend(secret_inputs.iter().map(|&v| Fp::from_u64(v)));
    // Ordered comparisons decompose values whose bits the prover must supply. Extend the
    // witness with the advice region and fill it from an evaluation run of the program.
    if compiled.n_advice > 0 {
        inputs.extend(core::iter::repeat(Fp::ZERO).take(compiled.n_advice as usize));
        fill_advice(&compiled, &mut inputs, public_inputs.len())?;
    }
    run_and_prove(&compiled.ops, &inputs, public_inputs.len())
}

/// Compile zkolang source with no public inputs, then prove and verify it.
pub fn prove_source(src: &str) -> Result<Report, RunError> {
    prove_source_with_inputs(src, &[])
}
