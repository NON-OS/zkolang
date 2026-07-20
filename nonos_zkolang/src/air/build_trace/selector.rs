/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The selector column an opcode tag sets.

use super::super::layout::*;
use crate::trace::OpTag;

/// The one-hot selector column index for an opcode tag.
pub(super) fn selector_of(op: OpTag) -> usize {
    match op {
        OpTag::Imm => S_IMM,
        OpTag::Add => S_ADD,
        OpTag::Sub => S_SUB,
        OpTag::Mul => S_MUL,
        OpTag::Inv => S_INV,
        OpTag::Eq => S_EQ,
        OpTag::Sel => S_SEL,
        OpTag::Bool => S_BOOL,
        OpTag::Assert => S_ASSERT,
        OpTag::Inp => S_INP,
        OpTag::Out => S_OUT,
        OpTag::Halt => S_HALT,
    }
}
