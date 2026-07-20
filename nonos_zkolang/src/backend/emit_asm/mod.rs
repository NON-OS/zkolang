/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The native x86_64 back-end: a program becomes a self-contained assembly file that
//! computes over the Goldilocks field, reads its inputs from the command line, and
//! prints its outputs. The field prelude and inverse are hand-written assembly; the
//! program body is one instruction sequence per opcode. Assembled with any C compiler
//! it runs as native code, and its result matches the proven trace.

mod data;
mod field;
mod header;
mod inv;
mod io;
mod op;
mod program;

pub use program::to_asm;
