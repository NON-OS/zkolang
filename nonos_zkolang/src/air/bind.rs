/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Bind one row's public input or output.

use nonos_stark::field::Fp;

use super::error::BuildError;
use super::layout::{A, IMM};
use crate::isa::Op;

/// Record the public binding a row induces and report whether it halts. A public
/// input is pinned to its committed value; a secret input has no value and is left
/// unbound; an output must have a supplied value.
pub(super) fn bind_op(
    row: usize,
    op: Op,
    public_inputs: &[Fp],
    public_outputs: &[Fp],
    binds: &mut alloc::vec::Vec<(usize, usize, Fp)>,
) -> Result<bool, BuildError> {
    match op {
        Op::Inp { idx, .. } => {
            if let Some(&v) = public_inputs.get(idx as usize) {
                binds.push((IMM, row, v));
            }
        }
        Op::Out { idx, .. } => {
            let v = public_outputs
                .get(idx as usize)
                .copied()
                .ok_or(BuildError::MissingPublicOutput { idx })?;
            binds.push((A, row, v));
        }
        Op::Halt => return Ok(true),
        _ => {}
    }
    Ok(false)
}
