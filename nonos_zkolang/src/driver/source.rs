/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The source-level entry points: compile first, then prove. These are the
//! functions a caller usually reaches for. Public inputs are supplied in
//! declaration order; a private witness feeds the run without entering the
//! public statement.

use alloc::vec::Vec;

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::advice::fill_advice;
use super::pipeline::{run_and_prove, run_and_prove_hidden};
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
    let (ops, inputs) = compile_and_bind(src, public_inputs, secret_inputs)?;
    run_and_prove(&ops, &inputs, public_inputs.len())
}

/// The same, hiding the witness. Each trace column is blinded from the private
/// `seed`, so the proof reveals only the public statement, the program commitment,
/// its inputs, and its outputs, not the secret witness or anything else about the
/// run. A fresh `seed` per proof makes each proof fresh. Errors with
/// `TraceTooSmallToHide` for a trace below the query-count blinding floor; the
/// transfer circuits size well above it.
pub fn prove_source_with_witness_zk(
    src: &str,
    public_inputs: &[u64],
    secret_inputs: &[u64],
    seed: &[Fp; RATE],
) -> Result<Report, RunError> {
    let (ops, inputs) = compile_and_bind(src, public_inputs, secret_inputs)?;
    run_and_prove_hidden(&ops, &inputs, public_inputs.len(), seed)
}

/// Compile the source and build the full input vector: the public prefix, the
/// private witness, and the filled comparison advice. Shared by the plain and the
/// hidden prove entries so the two bind an identical run.
fn compile_and_bind(
    src: &str,
    public_inputs: &[u64],
    secret_inputs: &[u64],
) -> Result<(Vec<crate::isa::Op>, Vec<Fp>), RunError> {
    let compiled = compile_source_full(src).map_err(RunError::Compile)?;
    let mut inputs: Vec<Fp> = public_inputs.iter().map(|&v| Fp::from_u64(v)).collect();
    inputs.extend(secret_inputs.iter().map(|&v| Fp::from_u64(v)));
    // Ordered comparisons decompose values whose bits the prover must supply. Extend the
    // witness with the advice region and fill it from an evaluation run of the program.
    if compiled.n_advice > 0 {
        inputs.resize(inputs.len() + compiled.n_advice as usize, Fp::ZERO);
        fill_advice(&compiled, &mut inputs, public_inputs.len())?;
    }
    Ok((compiled.ops, inputs))
}

/// Compile zkolang source with no public inputs, then prove and verify it.
pub fn prove_source(src: &str) -> Result<Report, RunError> {
    prove_source_with_inputs(src, &[])
}

/// Run a compiled program on public and secret inputs and return its outputs, without
/// proving. Used to check that a transform is behavior-preserving by comparing two
/// compilations of the same source. It does not fill comparison advice, so it is for
/// programs without ordered comparison.
pub fn evaluate(
    program: &[crate::isa::Op],
    public: &[u64],
    secret: &[u64],
) -> Result<Vec<u64>, RunError> {
    let mut inputs: Vec<Fp> = public.iter().map(|&v| Fp::from_u64(v)).collect();
    inputs.extend(secret.iter().map(|&v| Fp::from_u64(v)));
    let trace = crate::vm::Vm::new()
        .run(program, &inputs, public.len())
        .map_err(RunError::Execute)?;
    Ok(trace.public_outputs.iter().map(|f| f.value()).collect())
}
