/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The pipeline of edition 2026: a checked program lowered to SSA, optimized, its gadgets
 * expanded, allocated to registers and verified; and the witness a run of it needs.
 */

pub mod abi;
mod backend;
mod build;
mod build_diag;
mod build_lower;
mod execute;
mod run;
mod values;
mod witness;

pub use backend::{backend, BackendError, Compiled, MIN_ROWS};
pub use build::{build, Built, MAX_ROWS};
pub use run::{prove, run, Proved, RunFailure};
pub use witness::witness;
