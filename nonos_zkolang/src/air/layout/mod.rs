/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The trace column layout and the AIR's fixed sizes, in one place so every file
//! names a column by the same constant. Split into the step columns, the periodic
//! wiring columns, and the dimensions.

mod dims;
mod periodic;
mod step;

pub use step::TRACE_WIDTH;

pub(crate) use dims::{DEGREE, NUM_TRANSITION, WINDOW};
pub(crate) use periodic::{NUM_PERIODIC, READA_P, READB_P, READC_P, WRITE_P};
pub(crate) use step::{
    A, AUX, B, C, CLK, D, IMM, RF_BASE, S_ADD, S_ASSERT, S_BOOL, S_EQ, S_HALT, S_IMM, S_INP, S_INV,
    S_MUL, S_OUT, S_SEL, S_SUB,
};
