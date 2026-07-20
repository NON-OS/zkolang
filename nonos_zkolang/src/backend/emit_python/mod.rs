/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The Python back-end: a program becomes a module with a `run(inputs)` function
//! that returns the outputs, over Python's arbitrary-precision integers. This lets a
//! zKolang program be called from Python without a prover, and the result matches the
//! proven trace. Split into the field prelude, the module emitter, and the per-opcode
//! statement.

mod op;
mod prelude;
mod program;

pub use program::to_python;
