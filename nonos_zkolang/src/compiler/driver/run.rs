/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Running a built program on typed inputs, and proving the run. A proof hides the witness:
 * it shows the public inputs and the result and nothing else about the secret inputs.
 */

use alloc::vec::Vec;

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;

use super::abi::AbiError;
use super::build::Built;
use super::execute::execute;
use crate::compiler::interp::Failure;
use crate::driver::{prove_program_hidden, Report, RunError};

/** Why a run did not end in a result, or a proof did not end verified. */
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunFailure {
    /** The public inputs (`false`) or the secret ones do not fit the ABI. */
    Inputs(AbiError, bool),
    /** The program fails on these inputs, as the reference run shows. */
    Fails(Failure),
    /** The prover stopped. */
    Prove(RunError),
    /** The compiled program and the reference run disagree: a compiler bug. */
    Disagree,
    /** The proof was made and did not verify: a prover bug. */
    Unverified,
}

/** A verified proof's report, and the result it proves, one value per leaf. */
#[derive(Clone, Debug)]
pub struct Proved {
    pub report: Report,
    pub outputs: Vec<i128>,
}

/** Run `b` on `public` and `secret` leaf values: its result, one value per leaf. */
pub fn run(b: &Built, public: &[i128], secret: &[i128]) -> Result<Vec<i128>, RunFailure> {
    execute(b, public, secret).map(|e| e.outputs)
}

/**
 * Run `b`, prove the run hiding its witness with the blinding expanded from the private
 * `seed`, and verify the proof. A fresh seed per proof makes each proof fresh.
 */
pub fn prove(
    b: &Built,
    public: &[i128],
    secret: &[i128],
    seed: &[Fp; RATE],
) -> Result<Proved, RunFailure> {
    let e = execute(b, public, secret)?;
    let report = prove_program_hidden(&b.compiled.machine.ops, &e.full, e.n_public, seed)
        .map_err(RunFailure::Prove)?;
    if !report.verified {
        return Err(RunFailure::Unverified);
    }
    Ok(Proved {
        report,
        outputs: e.outputs,
    })
}
