/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The native C back-end: a program becomes a self-contained C source file that
//! computes over the Goldilocks field, reads its inputs from the command line, and
//! prints its outputs. Compiled with any C compiler it runs as native code, and its
//! result matches the proven trace. Split into the field prelude, the program
//! emitter, and the per-opcode statement.

mod op;
mod prelude;
mod program;

pub use program::to_c;
