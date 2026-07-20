/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The per-row wiring: which register a row writes and which registers its three
//! read ports name. This is the public data flow of the program, and it becomes
//! the periodic one-hot columns the register binding reads. It is derived purely
//! from the opcode, because register indices are compile-time constants.

use crate::isa::Op;

/// The data flow of one row. A `None` port reads nothing and pins its operand to
/// zero; a `None` write leaves every register unchanged that step.
#[derive(Clone, Copy)]
pub(super) struct WireRow {
    pub(super) write: Option<u8>,
    pub(super) read_a: Option<u8>,
    pub(super) read_b: Option<u8>,
    pub(super) read_c: Option<u8>,
}

impl WireRow {
    /// A row that wires nothing, used for halt and padding rows.
    pub(super) const EMPTY: WireRow = WireRow {
        write: None,
        read_a: None,
        read_b: None,
        read_c: None,
    };

    /// The wiring an opcode induces. Out-of-scope and halt rows wire nothing.
    pub(super) fn of(op: &Op) -> WireRow {
        match *op {
            Op::Imm { d, .. } => WireRow {
                write: Some(d),
                ..WireRow::EMPTY
            },
            Op::Inp { d, .. } => WireRow {
                write: Some(d),
                ..WireRow::EMPTY
            },
            Op::Add { d, a, b } | Op::Sub { d, a, b } | Op::Mul { d, a, b } => WireRow {
                write: Some(d),
                read_a: Some(a),
                read_b: Some(b),
                read_c: None,
            },
            Op::Inv { d, a } => WireRow {
                write: Some(d),
                read_a: Some(a),
                ..WireRow::EMPTY
            },
            Op::Eq { d, a, b } => WireRow {
                write: Some(d),
                read_a: Some(a),
                read_b: Some(b),
                read_c: None,
            },
            Op::Sel { d, c, a, b } => WireRow {
                write: Some(d),
                read_a: Some(a),
                read_b: Some(b),
                read_c: Some(c),
            },
            Op::Bool { a } | Op::Assert { a } => WireRow {
                read_a: Some(a),
                ..WireRow::EMPTY
            },
            Op::Out { a, .. } => WireRow {
                read_a: Some(a),
                ..WireRow::EMPTY
            },
            _ => WireRow::EMPTY,
        }
    }
}
