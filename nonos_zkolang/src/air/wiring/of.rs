/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The wiring an opcode induces.

use super::WireRow;
use crate::isa::Op;

impl WireRow {
    /// The write and read ports an opcode names. Out-of-scope and halt rows wire
    /// nothing. Opcodes with the same port shape share an arm.
    pub(in crate::air) fn of(op: &Op) -> WireRow {
        let (write, read_a, read_b, read_c) = match *op {
            Op::Imm { d, .. } | Op::Inp { d, .. } => (Some(d), None, None, None),
            Op::Add { d, a, b }
            | Op::Sub { d, a, b }
            | Op::Mul { d, a, b }
            | Op::Eq { d, a, b } => (Some(d), Some(a), Some(b), None),
            Op::Inv { d, a } => (Some(d), Some(a), None, None),
            Op::Sel { d, c, a, b } => (Some(d), Some(a), Some(b), Some(c)),
            Op::Bool { a } | Op::Assert { a } | Op::Out { a, .. } => (None, Some(a), None, None),
            _ => (None, None, None, None),
        };
        WireRow {
            write,
            read_a,
            read_b,
            read_c,
        }
    }
}
