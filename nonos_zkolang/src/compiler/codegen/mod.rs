/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Code generation: a machine-level SSA program written as machine instructions, with its
 * values assigned to the 32 registers. Straight-line code has no branches, so allocation
 * is one walk. A row reading a secret input or an advice slot is not pinned to any value,
 * so two reads of one slot could differ: each is read once, at its first use, and kept in
 * a register until its last. Only a constant or a public input is brought back after its
 * register is taken; when every register holds a value that cannot be, compilation stops.
 * `verify` replays the instructions against the program and rejects any that does not
 * compute what it claims or that reads a secret input or advice slot twice.
 */

mod emit;
mod machine;
mod op_of;
mod pad;
mod regs;
mod reload;
mod state;
mod verify;
mod verify_def;
mod verify_done;
mod verify_op;
mod verify_read;

pub use emit::codegen;
pub use machine::{CodegenError, Machine, Origin};
pub use pad::pad;
pub use verify::{verify, VerifyError};
