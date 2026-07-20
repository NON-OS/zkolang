/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Lay a run out as the flat trace matrix, one AIR row per VM row, padded with halts.

mod columns;
mod operands;
mod row;
mod selector;

use alloc::vec;
use alloc::vec::Vec;

use nonos_stark::field::Fp;

use super::error::BuildError;
use super::layout::TRACE_WIDTH;
use super::step_air::StepAir;
use crate::isa::REGS;
use crate::trace::Trace;

impl StepAir {
    /// Lay the run out, replay the register file, and pad with halt rows. The clock
    /// column is the row index, so ordering holds across the padding.
    pub fn build_trace(&self, trace: &Trace) -> Result<Vec<Fp>, BuildError> {
        let t = 1usize << self.log_t;
        let n = trace.rows.len();
        if n > t {
            return Err(BuildError::TooLong { rows: n, cap: t });
        }
        let mut flat = vec![Fp::ZERO; t * TRACE_WIDTH];
        let mut regfile = [Fp::ZERO; REGS];
        for i in 0..t {
            self.fill_row(&mut flat, i, n, trace, &mut regfile);
        }
        Ok(flat)
    }
}
