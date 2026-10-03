/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The point a zKølang format 7 proof is made at, and the trace size its blinding needs.
 * The query shape is STARKs' shape A, the one its pool proves at, on the launch
 * transcript with its DEEP and folding grinds; the blowup is every shipped statement's.
 */

use nonos_stark::air::{blinding_degree, blinding_fits, Air};
use nonos_stark::fri::FRI_FOLD_LOG;
use nonos_zkolang::{Op, StepAir};

/** Queries, and bits of query grind: STARKs shape A. */
pub const QUERIES: usize = 19;
pub const GRIND_BITS: u32 = 28;
/** Extra blowup bits over the minimal evaluation domain. */
pub const EXTRA_BLOWUP_BITS: u32 = 5;
/** Base-field lanes of a copy challenge: two, the launch transcript's rule set. */
pub const LANES: usize = 2;
/** The longest proof a statement accepts, bounding what a verifier reads. */
pub const MAX_PROOF_BYTES: usize = 1 << 20;

/** The step AIR's trace width, window and constraint degree. */
pub fn dims() -> (usize, usize, usize) {
    let air = StepAir::for_key(&[Op::Halt], 1).expect("a one-instruction program");
    (
        air.trace_width(),
        air.window_size(),
        air.constraint_degree(),
    )
}

/** The blinding degree of each column: one coefficient past every value a proof opens. */
pub fn blind_degree() -> usize {
    blinding_degree(QUERIES, dims().1, 1 << FRI_FOLD_LOG)
}

/** The fewest trace rows, as a power of two, whose composition admits that blinding. */
pub fn min_log_t() -> u32 {
    let (_, window, degree) = dims();
    let fits = |lg: &u32| blinding_fits(blind_degree(), degree, 1 << lg, window);
    (1..=24).find(fits).unwrap_or(24)
}
