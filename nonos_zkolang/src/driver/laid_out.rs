/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A run laid out for a prover of another proof format. */

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::prepare::prepare;
use super::RunError;
use crate::air::StepAir;
use crate::isa::Op;

/**
 * The AIR of a run of `program` on `inputs`, its trace row-major, and the public
 * statement its proof binds: the program commitment's four limbs, the trace length, the
 * `n_public` public inputs, then the outputs. Bound as this crate's own proofs are, the
 * trace sized to at least `2^min_log_t` rows.
 */
pub(crate) fn laid_out(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
    min_log_t: u32,
) -> Result<(StepAir, Vec<Fp>, Vec<Fp>), RunError> {
    let p = prepare(program, inputs, n_public, min_log_t)?;
    Ok((p.air, p.flat, p.publics))
}
