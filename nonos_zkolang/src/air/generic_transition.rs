/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/
use super::step_air::StepAir;
use alloc::vec::Vec;
use nonos_stark::air::GenericTransition;
use nonos_stark::field::Felt;

impl GenericTransition for StepAir {
    fn transition_gen<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        self.transition_over(window, periodic)
    }
}
