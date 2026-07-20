/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The VM instruction set. One flat opcode per VM step, chosen so each maps to a
//! small, fixed set of algebraic constraints in the step AIR. There are no
//! data-dependent jumps: control is selection, not branching, so the trace shape is a
//! function of the program and not the witness. Register operands are indices in
//! `0..REGS`.

mod op;

pub use op::Op;

/// Register file size. Fixed so the AIR's register columns are a constant width.
/// Thirty-two lanes hold a width-12 permutation and its mixing matrix, which needs
/// the old lanes live while the new ones are built. Every index is a full byte.
pub const REGS: usize = 32;

/// A compiled program: a flat instruction list. Bounded loops are unrolled by the
/// front-end before they reach here, so the list length is the step count.
pub type Program = [Op];
