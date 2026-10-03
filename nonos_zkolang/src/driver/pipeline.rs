/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The shared run-and-prove pipeline.

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::prepare::{prepare, Prepared};
use super::prover::{prove_verify, prove_verify_zk};
use super::{Report, RunError};
use crate::air::TRACE_WIDTH;
use crate::commit;
use crate::isa::Op;

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
    let p = prepare(program, inputs, n_public, 1)?;
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
    let p = prepare(program, inputs, n_public, 1)?;
    let verified = prove_verify_zk(&p.air, &p.flat, &p.publics, seed).ok_or(
        RunError::TraceTooSmallToHide {
            log_trace_len: p.log_trace_len,
        },
    )?;
    Ok(report(&p, program, verified))
}
