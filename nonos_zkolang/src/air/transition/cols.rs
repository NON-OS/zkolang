/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The window columns of the current row and the next row's clock.

use nonos_stark::field::Felt;

pub(super) struct Cols<F: Felt> {
    pub(super) clk: F,
    pub(super) s_imm: F,
    pub(super) s_add: F,
    pub(super) s_sub: F,
    pub(super) s_mul: F,
    pub(super) s_inv: F,
    pub(super) s_eq: F,
    pub(super) s_sel: F,
    pub(super) s_bool: F,
    pub(super) s_assert: F,
    pub(super) s_inp: F,
    pub(super) s_out: F,
    pub(super) s_halt: F,
    pub(super) a: F,
    pub(super) b: F,
    pub(super) c: F,
    pub(super) d: F,
    pub(super) imm: F,
    pub(super) aux: F,
    pub(super) next_clk: F,
}
