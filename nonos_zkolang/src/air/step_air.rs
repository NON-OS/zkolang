/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The AIR value: the padded trace length, the program's public wiring, and the
//! boundary values it binds. The methods that build and evaluate it live in the
//! sibling files; this file is just the state they share.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::wiring::WireRow;

/// The step AIR over a trace of `2^log_t` rows, carrying the public data-flow
/// wiring of the program it proves and the public input and output values it
/// binds.
pub struct StepAir {
    pub(super) log_t: u32,
    pub(super) wiring: Vec<WireRow>,
    /// Boundary triples binding public inputs and outputs: (column, row, value).
    pub(super) public_bindings: Vec<(usize, usize, Fp)>,
}
