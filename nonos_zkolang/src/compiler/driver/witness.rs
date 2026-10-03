/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The witness of a run: the inputs, then every advice value, each found by its hint from
 * a run of the machine-level SSA program. The machine checks every constraint over it.
 */

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::backend::Compiled;
use crate::compiler::ssa::eval::{eval, SsaFailure};
use crate::compiler::ssa::eval_hint::hint;

/** The full input vector of a run of `c` on `inputs`, or where the program fails. */
pub fn witness(c: &Compiled, inputs: &[Fp]) -> Result<Vec<Fp>, SsaFailure> {
    let run = eval(&c.ssa, inputs)?;
    let mut all = inputs.to_vec();
    for h in &c.machine.advice {
        all.push(hint(*h, &run.values).unwrap_or(Fp::ZERO));
    }
    Ok(all)
}
