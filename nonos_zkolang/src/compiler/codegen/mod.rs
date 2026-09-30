/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Code generation: a machine-level SSA program written as machine instructions, with its
 * values assigned to the 32 registers. Straight-line code has no branches, so allocation
 * is one walk: a value no register can keep is dropped when it can be read again (a
 * constant, an input, an advice value) and otherwise copied into the advice, the copy
 * constrained equal to it, and read back from there. `verify` replays the instructions
 * against the program and rejects any that does not compute what it claims.
 */

mod emit;
mod machine;
mod op_of;
mod regs;
mod reload;
mod spill;
mod state;
mod verify;
mod verify_def;
mod verify_done;
mod verify_op;
mod verify_spill;

pub use emit::codegen;
pub use machine::{CodegenError, Machine, Origin};
pub use verify::{verify, VerifyError};
