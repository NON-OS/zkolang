/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The zkolang VM executor. It runs a compiled program on public and private
//! inputs and emits the execution trace the STARK proves. It never panics: a
//! malformed program is a typed error, and a violated constraint (a failed
//! assert, an inverse of zero, a non-boolean selector) is reported as
//! `Unprovable`, the honest result, because such a trace has no proof.
//!
//! The executor is split so each file carries one concern: the error type, the
//! machine state and its register access, the run loop, and the per-opcode step.

mod error;
mod machine;
mod run;
mod step;

pub use error::ProveError;
pub use machine::Vm;
