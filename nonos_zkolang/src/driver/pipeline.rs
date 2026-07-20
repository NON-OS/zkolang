/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The shared run-and-prove pipeline.

use nonos_stark::field::Fp;

use super::prover::prove_verify;
use super::publics::build_publics;
use super::{choose_log_t, Report, RunError};
use crate::air::{StepAir, TRACE_WIDTH};
use crate::commit;
use crate::isa::Op;
use crate::vm::Vm;

/// `inputs` is the public prefix (`n_public` values) then the private witness; only
/// the public prefix enters the bound statement.
pub(super) fn run_and_prove(program: &[Op], inputs: &[Fp], n_public: usize) -> Result<Report, RunError> {
    let mut vm = Vm::new();
    let trace = vm.run(program, inputs, n_public).map_err(RunError::Execute)?;
    let steps = trace.rows.len();
    let log_trace_len = choose_log_t(steps).ok_or(RunError::ProgramTooLong { steps })?;
    let air = StepAir::compile(program, log_trace_len, &trace.public_inputs, &trace.public_outputs)
        .map_err(RunError::Layout)?;
    let flat = air.build_trace(&trace).map_err(RunError::Layout)?;
    let trace_len = 1usize << log_trace_len;
    let publics = build_publics(program, trace_len, &trace);
    let verified = prove_verify(&air, &flat, &publics);
    Ok(Report {
        verified,
        steps,
        log_trace_len,
        trace_len,
        trace_width: TRACE_WIDTH,
        outputs: trace.public_outputs.iter().map(|f| f.value()).collect(),
        program_commit: commit::commit(program),
    })
}
