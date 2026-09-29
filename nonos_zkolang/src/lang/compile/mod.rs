/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Lowering: the AST to a flat VM program. The language is single-assignment at the
//! source level, but the compiler reuses physical registers, which keeps register
//! pressure at expression depth. Register indices stay compile-time constants, all
//! the step AIR's register binding needs, so reuse is invisible to it. The work is
//! split by concern: the compiler state and allocator, the statement lowering, the
//! expression lowering, the constant tables, and the arrays.

mod array;
mod block_binds;
mod callees;
mod compiled;
mod compiler;
mod const_table;
mod count_inputs;
mod count_secrets;
mod duplicates;
mod expr;
mod live;
mod lower;
mod name_check;
mod recursion;
mod same_expr;
mod stmt;

pub use compiled::Compiled;
pub use lower::{compile, compile_full, compile_unoptimized};
