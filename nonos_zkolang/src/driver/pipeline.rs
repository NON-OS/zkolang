/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The shared run-and-prove pipeline.

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::prover::{prove_verify, prove_verify_zk};
use super::publics::build_publics;
use super::{choose_log_t, Report, RunError};
use crate::air::{StepAir, TRACE_WIDTH};
use crate::commit;
use crate::isa::Op;
use crate::trace::Trace;
use crate::vm::Vm;

/// The laid-out trace and its bound statement, shared by the plain and the hidden
/// prove paths so the two size and bind a program identically.
struct Prepared {
    air: StepAir,
    flat: alloc::vec::Vec<Fp>,
    publics: alloc::vec::Vec<Fp>,
    trace: Trace,
    steps: usize,
    log_trace_len: u32,
    trace_len: usize,
}

/// Run the program, size the trace, compile the AIR, and build the bound statement.
/// Everything up to the prove step, which the plain and hidden paths then do
/// differently.
fn prepare(program: &[Op], inputs: &[Fp], n_public: usize) -> Result<Prepared, RunError> {
    let mut vm = Vm::new();
    let trace = vm
        .run(program, inputs, n_public)
        .map_err(RunError::Execute)?;
    let steps = trace.rows.len();
    let log_trace_len = choose_log_t(steps).ok_or(RunError::ProgramTooLong { steps })?;
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

fn report(p: &Prepared, program: &[Op], verified: bool) -> Report {
    Report {
        verified,
        steps: p.steps,
        log_trace_len: p.log_trace_len,
        trace_len: p.trace_len,
        trace_width: TRACE_WIDTH,
        outputs: p.trace.public_outputs.iter().map(|f| f.value()).collect(),
        program_commit: commit::commit(program),
    }
}

/// `inputs` is the public prefix (`n_public` values) then the private witness; only
/// the public prefix enters the bound statement.
pub(super) fn run_and_prove(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
) -> Result<Report, RunError> {
    let p = prepare(program, inputs, n_public)?;
    let verified = prove_verify(&p.air, &p.flat, &p.publics);
    Ok(report(&p, program, verified))
}

/// The same run, hiding the witness: the trace columns are blinded from the private
/// `seed`, so the proof reveals nothing beyond the bound statement. `TraceTooSmallToHide`
/// when the trace cannot carry a blinding of the query count.
pub(super) fn run_and_prove_hidden(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
    seed: &[Fp; RATE],
) -> Result<Report, RunError> {
    let p = prepare(program, inputs, n_public)?;
    let verified = prove_verify_zk(&p.air, &p.flat, &p.publics, seed).ok_or(
        RunError::TraceTooSmallToHide {
            log_trace_len: p.log_trace_len,
        },
    )?;
    Ok(report(&p, program, verified))
}
