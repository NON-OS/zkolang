/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A zeroed row at a given clock.

use nonos_stark::field::Fp;

use super::{OpTag, Row};

impl Row {
    /// A zeroed row at a given clock, filled in by the executor per opcode.
    pub fn at(clk: u64) -> Row {
        Row {
            clk,
            op: OpTag::Halt,
            ra: Fp::ZERO,
            rb: Fp::ZERO,
            rc: Fp::ZERO,
            rd: Fp::ZERO,
            imm: Fp::ZERO,
            aux: Fp::ZERO,
        }
    }
}
