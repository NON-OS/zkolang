/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Fill the step columns of one row: the selector and the operands.

use nonos_stark::field::Fp;

use super::super::step_air::StepAir;
use super::selector::selector_of;
use crate::trace::Row;

impl StepAir {
    /// Set the selector, then the operand columns. Register binding is threaded
    /// separately by the caller.
    pub(super) fn write_step_columns(flat: &mut [Fp], base: usize, row: &Row) {
        flat[base + selector_of(row.op)] = Fp::ONE;
        Self::write_operands(flat, base, row);
    }
}
