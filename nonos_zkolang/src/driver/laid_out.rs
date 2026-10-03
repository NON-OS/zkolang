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
 * `n_public` public inputs, then the outputs. Sized and bound as this crate's own proofs
 * are.
 */
pub(crate) fn laid_out(
    program: &[Op],
    inputs: &[Fp],
    n_public: usize,
) -> Result<(StepAir, Vec<Fp>, Vec<Fp>), RunError> {
    let p = prepare(program, inputs, n_public)?;
    Ok((p.air, p.flat, p.publics))
}
