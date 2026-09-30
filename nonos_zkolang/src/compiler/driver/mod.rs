/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The pipeline of edition 2026: a checked program lowered to SSA, optimized, its gadgets
 * expanded, allocated to registers and verified; and the witness a run of it needs.
 */

mod backend;
mod witness;

pub use backend::{backend, BackendError, Compiled};
pub use witness::witness;
