/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The one-call driver: source and inputs in, a proven-and-verified report out. The
//! whole pipeline behind one function, the surface a shell command or a capsule
//! calls. Nothing here panics; every failure is a typed `RunError`.

mod advice;
mod error;
mod log_t;
mod params;
mod pipeline;
mod prove;
mod prover;
mod publics;
mod report;
mod source;

pub use error::RunError;
pub use prove::prove_program;
pub use report::Report;
pub use source::{prove_source, prove_source_with_inputs, prove_source_with_witness};

pub(crate) use log_t::choose_log_t;
