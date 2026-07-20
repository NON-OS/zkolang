/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The Air trait impl, delegating to sibling helper files so each block stays small.

mod boundary;
mod ext;
mod periodic;

use alloc::vec::Vec;

use nonos_stark::air::Air;
use nonos_stark::field::Fp;

use super::layout::{DEGREE, NUM_TRANSITION, TRACE_WIDTH, WINDOW};
use super::step_air::StepAir;

impl Air for StepAir {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }
    fn trace_width(&self) -> usize {
        TRACE_WIDTH
    }
    fn window_size(&self) -> usize {
        WINDOW
    }
    fn constraint_degree(&self) -> usize {
        DEGREE
    }
    fn num_transition(&self) -> usize {
        NUM_TRANSITION
    }
    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        self.periodic_columns_impl()
    }
    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }
    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        self.boundary_impl()
    }
}
