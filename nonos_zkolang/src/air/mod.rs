/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zKolang step AIR: the branchless computational core with register binding and
//! public input and output binding. For a trace laid out one VM step per row it
//! proves that every row names one opcode and computes it, that every operand is the
//! live value of the register it names and every register carries forward until
//! written, that inputs and outputs match the committed public values, and that the
//! rows are clock-ordered. Register indices are compile-time, so the data flow is a
//! public circuit carried as periodic one-hot columns. The work is split by concern,
//! one file per piece.

mod air_impl;
mod bind;
mod build_trace;
mod compile;
mod error;
mod for_key;
mod layout;
mod pad;
mod step_air;
mod transition;
mod wiring;

pub use error::BuildError;
pub use layout::TRACE_WIDTH;
pub use step_air::StepAir;
