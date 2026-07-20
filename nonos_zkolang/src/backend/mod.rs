/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Back-ends: the same compiled program, emitted to other targets. The front-end
//! turns zKolang source into a flat op list, and that list is target-independent, so
//! a program that the STARK proves can equally be emitted as native C or as Python
//! and run without a prover. One source, many targets: the proven trace is one of
//! them, not the only one.
//!
//! Every target computes over the same Goldilocks field, so a program produces the
//! identical outputs whichever back-end runs it, which is what the host suite checks.

mod emit_c;
mod emit_python;

pub use emit_c::to_c;
pub use emit_python::to_python;

use crate::isa::Op;

// The number of public and private inputs a program reads, one past the highest
// input index, so an emitted target sizes its input array to match the VM.
pub(super) fn n_inputs(program: &[Op]) -> usize {
    program
        .iter()
        .filter_map(|op| match op {
            Op::Inp { idx, .. } => Some(*idx as usize + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0)
}

// The number of public outputs, one past the highest output index.
pub(super) fn n_outputs(program: &[Op]) -> usize {
    program
        .iter()
        .filter_map(|op| match op {
            Op::Out { idx, .. } => Some(*idx as usize + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0)
}
