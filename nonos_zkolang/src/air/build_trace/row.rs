/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Fill one trace row and advance the register file.

use nonos_stark::field::Fp;

use super::super::layout::{CLK, RF_BASE, S_HALT, TRACE_WIDTH};
use super::super::step_air::StepAir;
use crate::isa::REGS;
use crate::trace::Trace;

impl StepAir {
    /// Fill row `i`: the clock, the step columns (or a halt for padding), the register
    /// file before the row, then this row's write so the next row sees it.
    pub(super) fn fill_row(
        &self,
        flat: &mut [Fp],
        i: usize,
        n: usize,
        trace: &Trace,
        regfile: &mut [Fp; REGS],
    ) {
        let base = i * TRACE_WIDTH;
        flat[base + CLK] = Fp::from_u64(i as u64);
        if i < n {
            Self::write_step_columns(flat, base, &trace.rows[i]);
        } else {
            flat[base + S_HALT] = Fp::ONE;
        }
        for (k, value) in regfile.iter().enumerate() {
            flat[base + RF_BASE + k] = *value;
        }
        if i < n {
            if let Some(k) = self.wiring[i].write {
                regfile[k as usize] = trace.rows[i].rd;
            }
        }
    }
}
