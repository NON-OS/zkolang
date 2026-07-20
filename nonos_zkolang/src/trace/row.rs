/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! One VM step. The `op` tag selects which constraint set is active for the row.

use nonos_stark::field::Fp;

use super::OpTag;

#[derive(Clone, Copy, Debug)]
pub struct Row {
    /// Step counter; row 0 is the boundary.
    pub clk: u64,
    /// Opcode tag for the selector column.
    pub op: OpTag,
    /// Register value read on port a.
    pub ra: Fp,
    /// Register value read on port b.
    pub rb: Fp,
    /// Register value read on port c.
    pub rc: Fp,
    /// Register value written.
    pub rd: Fp,
    /// Immediate operand, when the op carries one.
    pub imm: Fp,
    /// Auxiliary witness: the inverse for Inv and Eq, the tested value for Bool and
    /// Assert. Zero when unused.
    pub aux: Fp,
}
