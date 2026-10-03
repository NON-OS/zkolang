/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A run laid out for the prover: the trace sized, the AIR compiled, the statement built.
 */

use nonos_stark::field::Fp;

use super::publics::build_publics;
use super::{choose_log_t, RunError};
use crate::air::StepAir;
use crate::isa::Op;
use crate::trace::Trace;
use crate::vm::Vm;

/**
 * The laid-out trace and its bound statement, shared by the plain and the hidden
 * prove paths so the two size and bind a program identically.
 */
pub(super) struct Prepared {
    pub(super) air: StepAir,
    pub(super) flat: alloc::vec::Vec<Fp>,
    pub(super) publics: alloc::vec::Vec<Fp>,
    pub(super) trace: Trace,
    pub(super) steps: usize,
    pub(super) log_trace_len: u32,
    pub(super) trace_len: usize,
}

/**
 * Run the program, size the trace to the fewest rows that hold it and at least
 * `2^min_log_t`, compile the AIR, and build the bound statement. Everything up to the
 * prove step, which the plain and hidden paths then do differently.
 */
pub(super) fn prepare(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
    min_log_t: u32,
) -> Result<Prepared, RunError> {
    let mut vm = Vm::new();
    let trace = vm
        .run(program, inputs, n_public)
        .map_err(RunError::Execute)?;
    let steps = trace.rows.len();
    let log_trace_len = choose_log_t(steps)
        .map(|lg| lg.max(min_log_t))
        .ok_or(RunError::ProgramTooLong { steps })?;
    let air = StepAir::compile(
        program,
        log_trace_len,
        &trace.public_inputs,
        &trace.public_outputs,
    )
    .map_err(RunError::Layout)?;
    let flat = air.build_trace(&trace).map_err(RunError::Layout)?;
    let trace_len = 1usize << log_trace_len;
    let publics = build_publics(program, trace_len, &trace);
    Ok(Prepared {
        air,
        flat,
        publics,
        trace,
        steps,
        log_trace_len,
        trace_len,
    })
}
