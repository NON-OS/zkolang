/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A run of a built program laid out for a prover of another proof format: the AIR, the
 * trace and the public statement, checked against the reference interpreter as `prove`
 * checks it.
 */

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::built::Built;
use super::execute::execute;
use super::run::RunFailure;
use crate::air::StepAir;
use crate::driver::laid_out;

/** A run's AIR, its trace and its public statement. */
pub struct Traced {
    /** The step AIR of the run, its wiring and public bindings. */
    pub air: StepAir,
    /** The trace, row-major, `air.trace_width()` cells a row. */
    pub trace: Vec<Fp>,
    /**
     * The public statement: the program commitment's four limbs, the trace length,
     * the public input slots, then the output slots.
     */
    pub publics: Vec<Fp>,
    /** How many public input slots the statement holds. */
    pub inputs: usize,
    /** The result, one value per leaf. */
    pub outputs: Vec<i128>,
}

/**
 * Run `b` on `public` and `secret` leaf values and lay the run out for a prover, the trace
 * sized to at least `2^min_log_t` rows, as a prover that blinds more than this crate's
 * own may need.
 */
pub fn traced(
    b: &Built,
    public: &[i128],
    secret: &[i128],
    min_log_t: u32,
) -> Result<Traced, RunFailure> {
    let e = execute(b, public, secret)?;
    let ops = &b.compiled.machine.ops;
    let laid = laid_out(ops, &e.full, e.n_public, min_log_t);
    let (air, trace, publics) = laid.map_err(RunFailure::Prove)?;
    Ok(Traced {
        air,
        trace,
        publics,
        inputs: e.n_public,
        outputs: e.outputs,
    })
}
