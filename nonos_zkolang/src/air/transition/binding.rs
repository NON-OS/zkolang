/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Register binding: each read port equals the register it names, and each register
//! carries forward unless this row writes it.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Felt;

use super::super::layout::{READA_P, READB_P, READC_P, RF_BASE, TRACE_WIDTH, WRITE_P};
use super::Cols;
use crate::isa::REGS;

pub(super) fn register_binding<F: Felt>(window: &[F], periodic: &[F], c: &Cols<F>) -> Vec<F> {
    let one = F::ONE;
    /*
     * Each read port is the dot product of the register file with a public one-hot column.
     * The port selectors READA, READB, READC ride the periodic columns, so they are fixed by
     * the program at compile time rather than chosen by the prover: this is where binding
     * stops needing a permutation argument. The dot product picks out exactly the named
     * register because the selector is one there and zero elsewhere.
     */
    let mut read_a = F::ZERO;
    let mut read_b = F::ZERO;
    let mut read_c = F::ZERO;
    for k in 0..REGS {
        let rf_k = window[RF_BASE + k];
        read_a = read_a + periodic[READA_P + k] * rf_k;
        read_b = read_b + periodic[READB_P + k] * rf_k;
        read_c = read_c + periodic[READC_P + k] * rf_k;
    }
    let mut cs = vec![c.a - read_a, c.b - read_b, c.c - read_c];
    /*
     * The update carries every register forward unchanged except the one this row writes.
     * With the write selector w_k public and one-hot, the next value is
     * (1 - w_k) * rf_k + w_k * d: the old value where w_k is zero, the result d where it is
     * one. The whole register file evolves by one linear constraint per lane, no grand
     * product, so the wiring of the program is a circuit the verifier reads rather than a
     * multiset it must trust.
     */
    for k in 0..REGS {
        let rf_k = window[RF_BASE + k];
        let rf_next_k = window[TRACE_WIDTH + RF_BASE + k];
        let w_k = periodic[WRITE_P + k];
        cs.push(rf_next_k - ((one - w_k) * rf_k + w_k * c.d));
    }
    cs
}
