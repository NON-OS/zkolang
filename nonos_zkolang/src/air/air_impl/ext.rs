/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The transition over the extension field.

use alloc::vec::Vec;

use nonos_stark::air::AirExt;
use nonos_stark::field::Fp2;

use super::super::step_air::StepAir;

impl AirExt for StepAir {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
