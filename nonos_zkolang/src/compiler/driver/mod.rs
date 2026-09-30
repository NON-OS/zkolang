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
mod built;
mod execute;
mod leaves;
mod run;
mod run_diag;
mod seed;
mod slots_enum;
mod slots_read;
mod slots_write;
mod test;
mod test_verdict;
mod value_of;
mod values;
mod witness;

pub use backend::{backend, BackendError, Compiled, MIN_ROWS};
pub use build::{build, MAX_ROWS};
pub use built::Built;
pub use run::{prove, run, Proved, RunFailure};
pub use run_diag::diagnose;
pub use seed::{seed_of, SEED_BYTES};
pub use test::{test, TestReport};
pub use witness::witness;
