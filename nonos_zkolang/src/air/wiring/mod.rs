/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The per-row wiring: which register a row writes and which its three read ports
//! name. This is the public data flow, and it becomes the periodic one-hot columns
//! the register binding reads. It is derived purely from the opcode.

mod of;

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
    pub(in crate::air) const EMPTY: WireRow = WireRow {
        write: None,
        read_a: None,
        read_b: None,
        read_c: None,
    };
}
