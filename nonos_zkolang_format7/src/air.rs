/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The step AIR as a format 7 prover takes it. The two-round prover commits the columns
 * below `region_width` first, draws the copy challenges, and commits the rest second;
 * every format 7 verifier requires at least one column in that second round. The step
 * AIR has no copy constraint and reads no challenge, so its last column is committed
 * second and nothing is filled between the rounds: the constraints, and what a proof
 * shows, are the step AIR's.
 */

use nonos_stark::air::{Air, AirExt, Permuted};
use nonos_stark::field::{Fp, Fp2};
use nonos_zkolang::StepAir;

use super::params::LANES;

/** The step AIR, its last column committed in the second round. */
pub struct Air7<'a>(pub &'a StepAir);

impl Air for Air7<'_> {
    fn log_trace_len(&self) -> u32 {
        self.0.log_trace_len()
    }
    fn trace_width(&self) -> usize {
        self.0.trace_width()
    }
    fn window_size(&self) -> usize {
        self.0.window_size()
    }
    fn constraint_degree(&self) -> usize {
        self.0.constraint_degree()
    }
    fn num_transition(&self) -> usize {
        self.0.num_transition()
    }
    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        self.0.periodic_columns()
    }
    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.0.transition(window, periodic)
    }
    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        self.0.boundary()
    }
}

impl AirExt for Air7<'_> {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.0.transition_ext(window, periodic)
    }
    fn challenge_lanes(&self) -> usize {
        LANES
    }
}

impl Permuted for Air7<'_> {
    fn region_width(&self) -> usize {
        self.0.trace_width() - 1
    }
    fn set_challenges(&mut self, _beta: Fp, _gamma: Fp) {}
    fn fill_products(&self, _trace: &mut [Fp]) {}
}
