/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Count a program's inputs.

use crate::isa::Op;

/// The number of public and private inputs a program reads, one past the highest
/// input index, so an emitted target sizes its input array to match the VM.
pub(crate) fn n_inputs(program: &[Op]) -> usize {
    program
        .iter()
        .filter_map(|op| match op {
            Op::Inp { idx, .. } => Some(*idx as usize + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0)
}
