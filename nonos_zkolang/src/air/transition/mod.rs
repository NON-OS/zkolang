/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The transition constraints, written once over any field so the base-field
//! composition and the extension-field out-of-domain evaluation share one
//! definition. Each returned value must be zero on every row. The groups are the
//! window columns, the selector well-formedness and clock step, the opcode gates, and
//! the register binding.

mod binding;
mod cols;
mod opcode;
mod read;
mod selector;
mod sum;

use alloc::vec::Vec;

use nonos_stark::field::Felt;

use binding::register_binding;
use cols::Cols;
use opcode::opcode_constraints;
use selector::selector_constraints;

use super::step_air::StepAir;

impl StepAir {
    /// The transition over any field. `window` is row-major and `periodic` holds the
    /// wiring one-hots at the current point. The groups concatenate in a fixed order,
    /// consistent between prove and verify.
    pub(in crate::air) fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let c = Cols::read(window);
        let mut cs = selector_constraints(&c);
        cs.extend(opcode_constraints(&c));
        cs.extend(register_binding(window, periodic, &c));
        cs
    }
}
