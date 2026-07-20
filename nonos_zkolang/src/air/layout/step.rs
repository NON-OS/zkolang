/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Step columns: a clock, twelve one-hot opcode selectors, three read operands, a
//! result, an immediate, an auxiliary witness, then the register file.

use crate::isa::REGS;

pub(crate) const CLK: usize = 0;
pub(crate) const S_IMM: usize = 1;
pub(crate) const S_ADD: usize = 2;
pub(crate) const S_SUB: usize = 3;
pub(crate) const S_MUL: usize = 4;
pub(crate) const S_INV: usize = 5;
pub(crate) const S_EQ: usize = 6;
pub(crate) const S_SEL: usize = 7;
pub(crate) const S_BOOL: usize = 8;
pub(crate) const S_ASSERT: usize = 9;
pub(crate) const S_INP: usize = 10;
pub(crate) const S_OUT: usize = 11;
pub(crate) const S_HALT: usize = 12;
pub(crate) const A: usize = 13;
pub(crate) const B: usize = 14;
pub(crate) const C: usize = 15;
pub(crate) const D: usize = 16;
pub(crate) const IMM: usize = 17;
pub(crate) const AUX: usize = 18;

/// The register file's first column; it holds the state before the row executes.
pub(crate) const RF_BASE: usize = 19;

/// The width of the step trace: the step columns plus the register file.
pub const TRACE_WIDTH: usize = RF_BASE + REGS;
