/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The periodic wiring columns.

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::super::layout::NUM_PERIODIC;
use super::super::step_air::StepAir;
use super::super::wiring::WireRow;
use crate::isa::REGS;

impl StepAir {
    /// The four ports in order (write, then the three reads); for each and each
    /// register, a column that is one on the rows that name that register.
    pub(super) fn periodic_columns_impl(&self) -> Vec<Vec<Fp>> {
        let t = 1usize << self.log_t;
        let mut cols: Vec<Vec<Fp>> = Vec::with_capacity(NUM_PERIODIC);
        let port = |w: &WireRow, which: usize| -> Option<u8> {
            match which {
                0 => w.write,
                1 => w.read_a,
                2 => w.read_b,
                _ => w.read_c,
            }
        };
        for which in 0..4 {
            for k in 0..REGS {
                let mut col = vec![Fp::ZERO; t];
                for (i, cell) in col.iter_mut().enumerate() {
                    if port(&self.wiring[i], which) == Some(k as u8) {
                        *cell = Fp::ONE;
                    }
                }
                cols.push(col);
            }
        }
        cols
    }
}
