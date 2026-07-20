/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The fixed-width byte record for one opcode.

use alloc::vec::Vec;

use crate::isa::Op;

/// Append one opcode's fixed-width record: a one-byte tag then its operands, so a
/// change to any instruction changes the bytes and the digest over them.
pub(super) fn encode_op(op: Op, out: &mut Vec<u8>) {
    match op {
        Op::Imm { d, v } => {
            out.push(0x00);
            out.push(d);
            out.extend_from_slice(&v.value().to_le_bytes());
        }
        Op::Add { d, a, b } => out.extend_from_slice(&[0x01, d, a, b]),
        Op::Sub { d, a, b } => out.extend_from_slice(&[0x02, d, a, b]),
        Op::Mul { d, a, b } => out.extend_from_slice(&[0x03, d, a, b]),
        Op::Inv { d, a } => out.extend_from_slice(&[0x04, d, a]),
        Op::Sel { d, c, a, b } => out.extend_from_slice(&[0x05, d, c, a, b]),
        Op::Eq { d, a, b } => out.extend_from_slice(&[0x06, d, a, b]),
        Op::Bool { a } => out.extend_from_slice(&[0x07, a]),
        Op::Assert { a } => out.extend_from_slice(&[0x08, a]),
        Op::Inp { d, idx } => {
            out.push(0x09);
            out.push(d);
            out.extend_from_slice(&idx.to_le_bytes());
        }
        Op::Out { a, idx } => {
            out.push(0x0a);
            out.push(a);
            out.extend_from_slice(&idx.to_le_bytes());
        }
        Op::Halt => out.push(0x0b),
    }
}
