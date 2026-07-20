/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The boundary constraints.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::super::layout::{A, AUX, B, C, CLK, D, IMM, RF_BASE, S_HALT};
use super::super::step_air::StepAir;
use crate::isa::REGS;

impl StepAir {
    /// The clock starts at zero, the final row is a clean halt with no operand data,
    /// every register starts at zero, and the public inputs and outputs are bound to
    /// the rows that read or expose them.
    pub(super) fn boundary_impl(&self) -> Vec<(usize, usize, Fp)> {
        let last = (1usize << self.log_t) - 1;
        let mut bnd = vec![
            (CLK, 0, Fp::ZERO),
            (S_HALT, last, Fp::ONE),
            (A, last, Fp::ZERO),
            (B, last, Fp::ZERO),
            (C, last, Fp::ZERO),
            (D, last, Fp::ZERO),
            (IMM, last, Fp::ZERO),
            (AUX, last, Fp::ZERO),
        ];
        for k in 0..REGS {
            bnd.push((RF_BASE + k, 0, Fp::ZERO));
        }
        bnd.extend_from_slice(&self.public_bindings);
        bnd
    }
}
