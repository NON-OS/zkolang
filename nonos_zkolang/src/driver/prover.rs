/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Prove a laid-out trace and verify it under the bound statement.

use nonos_stark::air::{
    blinding_poly, stark_prove_poseidon_ext_pub, stark_verify_poseidon_ext_pub, Air, Poseidon, RATE,
};
use nonos_stark::field::Fp;

use alloc::vec::Vec;

use super::params::{BLOWUP, GRIND, QUERIES};
use crate::air::StepAir;

/// Prove the trace with the public statement seeded into the transcript, then verify
/// the proof against the same statement. The hasher is fixed, so prove and verify
/// replay one transcript.
pub(super) fn prove_verify(air: &StepAir, flat: &[Fp], publics: &[Fp]) -> bool {
    let hasher = Poseidon::new(2, [Fp::ZERO; RATE]);
    let proof =
        stark_prove_poseidon_ext_pub(air, flat, QUERIES, GRIND, BLOWUP, &hasher, publics, &[]);
    stark_verify_poseidon_ext_pub(air, &proof, QUERIES, GRIND, BLOWUP, &hasher, publics)
}

/// The largest blinding degree the composition bound admits for a trace of length
/// `t` with the given constraint `degree` and `window`. Each column becomes degree
/// `t + b`, so the transition composition runs to `degree*(t + b) + (window - 1) - t`,
/// while the FRI bound is `B = (degree*t).next_power_of_two()`; the blinded quotient
/// stays low degree while that composition is below `B`, which rearranges to
/// `degree*b <= B + t - degree*t - window`.
fn max_blind_degree(t: usize, degree: usize, window: usize) -> usize {
    let degree = degree.max(1);
    let bound = (degree * t).next_power_of_two();
    (bound + t).saturating_sub(degree * t + window) / degree
}

/// Prove the trace hiding the witness, then verify under the same bound statement.
/// Each column is blinded by `r * Z_H` with `r` expanded from the prover's private
/// `seed`, so the query openings are jointly uniform and leak nothing; a fresh seed
/// per proof makes every proof fresh. The verifier is the plain one: blinding is
/// invisible to it.
///
/// `None` when the trace is too small to carry a blinding of degree `QUERIES`. Below
/// that floor the openings would not be jointly uniform over all `QUERIES` query
/// points, and a blinding shorter than the query count is not hiding, so the honest
/// answer is to refuse rather than sell a weaker proof as zero-knowledge. The floor
/// is `2t - WINDOW >= DEGREE * QUERIES`, first met at `log_t = 6`; the transfer
/// circuit sizes far above it.
pub(super) fn prove_verify_zk(
    air: &StepAir,
    flat: &[Fp],
    publics: &[Fp],
    seed: &[Fp; RATE],
) -> Option<bool> {
    let t = 1usize << air.log_trace_len();
    if max_blind_degree(t, air.constraint_degree(), air.window_size()) < QUERIES {
        return None;
    }
    let hasher = Poseidon::new(2, [Fp::ZERO; RATE]);
    let blind: Vec<Vec<Fp>> = (0..air.trace_width())
        .map(|c| blinding_poly(&hasher, seed, c, QUERIES))
        .collect();
    let proof =
        stark_prove_poseidon_ext_pub(air, flat, QUERIES, GRIND, BLOWUP, &hasher, publics, &blind);
    Some(stark_verify_poseidon_ext_pub(
        air, &proof, QUERIES, GRIND, BLOWUP, &hasher, publics,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * The StepAir shape the transfer prover runs on: constraint degree three, a
     * two-row window. The hiding floor is computed against these, so the test tracks
     * the real AIR without depending on the private layout constants.
     */
    const STEP_DEGREE: usize = 3;
    const STEP_WINDOW: usize = 2;

    /// The hiding floor is a property of the AIR shape and the query count, not of
    /// any particular run: a blinding of degree `QUERIES` fits the composition bound
    /// exactly from `log_t = 6` up, and not below. This pins the threshold the driver
    /// refuses under, so a change to the AIR degree or the query count that would
    /// silently move it is caught here.
    #[test]
    fn the_hiding_floor_is_log_t_six() {
        /*
         * t = 32 (log_t = 5): (2*32 - 2)/3 = 20 slots, short of the 32 the query
         * count needs, so hiding is refused.
         */
        assert!(
            max_blind_degree(1 << 5, STEP_DEGREE, STEP_WINDOW) < QUERIES,
            "log_t 5 must be below the hiding floor"
        );
        /*
         * t = 64 (log_t = 6): (2*64 - 2)/3 = 42 slots, room for the 32 query
         * openings with margin, so hiding is admitted.
         */
        assert!(
            max_blind_degree(1 << 6, STEP_DEGREE, STEP_WINDOW) >= QUERIES,
            "log_t 6 must clear the hiding floor"
        );
        // The floor holds monotonically above it: every larger trace has more room.
        for log_t in 6..=16u32 {
            assert!(
                max_blind_degree(1usize << log_t, STEP_DEGREE, STEP_WINDOW) >= QUERIES,
                "a trace at or above the floor must admit a query-count blinding"
            );
        }
    }
}
