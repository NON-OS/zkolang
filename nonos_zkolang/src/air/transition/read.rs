/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Read the window columns into `Cols`.

use nonos_stark::field::Felt;

use super::super::layout::*;
use super::Cols;

impl<F: Felt> Cols<F> {
    pub(super) fn read(w: &[F]) -> Cols<F> {
        Cols {
            clk: w[CLK],
            s_imm: w[S_IMM],
            s_add: w[S_ADD],
            s_sub: w[S_SUB],
            s_mul: w[S_MUL],
            s_inv: w[S_INV],
            s_eq: w[S_EQ],
            s_sel: w[S_SEL],
            s_bool: w[S_BOOL],
            s_assert: w[S_ASSERT],
            s_inp: w[S_INP],
            s_out: w[S_OUT],
            s_halt: w[S_HALT],
            a: w[A],
            b: w[B],
            c: w[C],
            d: w[D],
            imm: w[IMM],
            aux: w[AUX],
            next_clk: w[TRACE_WIDTH + CLK],
        }
    }
}
