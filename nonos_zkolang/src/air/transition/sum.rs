/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The sum of the selector columns, one when exactly one opcode is set.

use nonos_stark::field::Felt;

use super::Cols;

impl<F: Felt> Cols<F> {
    pub(super) fn selector_sum(&self) -> F {
        self.s_imm
            + self.s_add
            + self.s_sub
            + self.s_mul
            + self.s_inv
            + self.s_eq
            + self.s_sel
            + self.s_bool
            + self.s_assert
            + self.s_inp
            + self.s_out
            + self.s_halt
    }
}
