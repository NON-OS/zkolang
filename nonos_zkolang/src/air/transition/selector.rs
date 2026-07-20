/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Selector well-formedness and the clock step. Each selector is boolean, exactly one
//! is set, and the clock rises by one.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Felt;

use super::Cols;

pub(super) fn selector_constraints<F: Felt>(c: &Cols<F>) -> Vec<F> {
    let one = F::ONE;
    vec![
        c.s_imm * (c.s_imm - one),
        c.s_add * (c.s_add - one),
        c.s_sub * (c.s_sub - one),
        c.s_mul * (c.s_mul - one),
        c.s_inv * (c.s_inv - one),
        c.s_eq * (c.s_eq - one),
        c.s_sel * (c.s_sel - one),
        c.s_bool * (c.s_bool - one),
        c.s_assert * (c.s_assert - one),
        c.s_inp * (c.s_inp - one),
        c.s_out * (c.s_out - one),
        c.s_halt * (c.s_halt - one),
        c.selector_sum() - one,
        c.next_clk - c.clk - one,
    ]
}
