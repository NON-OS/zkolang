/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Fill the comparison advice. Each ordered comparison decomposes a value into bits the
//! prover supplies, but those bits depend on the run, so the driver evaluates the
//! program without enforcing constraints, reads the value each decomposition writes,
//! and sets its bits in the advice suffix of the witness. This repeats to a fixed
//! point, so a comparison whose operands depend on another comparison still settles.
//! The proving run then enforces every constraint over the filled witness, so soundness
//! rests on the constraints, never on how the bits were produced.

use nonos_stark::field::Fp;

use super::RunError;
use crate::lang::Compiled;
use crate::vm::Vm;

pub(super) fn fill_advice(
    compiled: &Compiled,
    inputs: &mut [Fp],
    n_public: usize,
) -> Result<(), RunError> {
    let base = inputs.len() - compiled.n_advice as usize;
    for _ in 0..8 {
        let trace = Vm::evaluator()
            .run(&compiled.ops, inputs, n_public)
            .map_err(RunError::Execute)?;
        let mut changed = false;
        for adv in &compiled.advice {
            let value = trace.rows[adv.value_op as usize].rd.value();
            for k in 0..adv.width as usize {
                let bit = Fp::from_u64((value >> k) & 1);
                let idx = base + adv.start as usize + k;
                if inputs[idx] != bit {
                    inputs[idx] = bit;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    Ok(())
}
