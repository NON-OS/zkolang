/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The SSA IR: a program as one straight line of instructions over field elements, each
 * defining at most one value. Calls are inlined, loops unrolled and branches guarded
 * before it, so it has no control flow. Machine-level instructions are one machine
 * instruction each; gadget-level ones stand for a constraint system the gadget expander
 * writes out, and name no advice of their own.
 */

mod builder;
pub mod eval;
mod eval_check;
pub mod eval_hint;
mod hint;
mod inst;
mod inst_map;
mod inst_operands;
mod program;
mod value;

pub use builder::Builder;
pub use hint::Hint;
pub use inst::Inst;
pub use program::Ssa;
pub use value::V;
