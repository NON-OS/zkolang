/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A full execution trace plus the public boundary the proof commits to.

use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::Row;

pub struct Trace {
    pub rows: Vec<Row>,
    pub public_inputs: Vec<Fp>,
    pub public_outputs: Vec<Fp>,
}
