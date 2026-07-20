/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A compiled program together with its advice plan. Ordered comparison needs the bit
//! decomposition of a value as a private witness the prover must supply. The compiler
//! cannot know those bits, they depend on the run, so it records, per decomposition,
//! the instruction whose written value is decomposed and where its bits live in the
//! advice suffix of the witness. The driver runs once to read the values, fills the
//! bits, and proves. The advice is a hidden suffix, never part of the public statement.

use alloc::vec::Vec;

use crate::isa::Op;

/// One decomposition the driver must fill: read the value written by instruction
/// `value_op`, take its low `width` bits, and place them at `start` in the advice
/// region of the witness.
#[derive(Clone, Copy, Debug)]
pub struct Advice {
    pub value_op: u32,
    pub start: u16,
    pub width: u8,
}

/// A program and the advice its comparisons need. `n_advice` is the total number of
/// advice bits, the length of the witness suffix past the user secrets.
#[derive(Clone, Debug)]
pub struct Compiled {
    pub ops: Vec<Op>,
    pub advice: Vec<Advice>,
    pub n_advice: u16,
}
