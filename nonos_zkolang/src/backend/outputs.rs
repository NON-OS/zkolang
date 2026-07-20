/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Count a program's outputs.

use crate::isa::Op;

/// The number of public outputs, one past the highest output index.
pub(crate) fn n_outputs(program: &[Op]) -> usize {
    program
        .iter()
        .filter_map(|op| match op {
            Op::Out { idx, .. } => Some(*idx as usize + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0)
}
