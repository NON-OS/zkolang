// NONOS Operating System (AGPL-3.0-or-later)
//! The generic compose region proven inside the recursion crate on a real
//! zkolang step inner. `ComposeCheckGen<StepAir>` recomputes every step
//! transition from the proof's out-of-domain frame and batches it to the claimed
//! composition. This is the crate-boundary proof — the recursion crate resolving
//! and gating the generic compose against `nonos_zkolang`'s StepAir — and the
//! first L4 region wired in generic form. The honest composition satisfies every
//! constraint; a tampered frame cell breaks one.

use crate::crypto::stark::air::Air;
use crate::crypto::stark::field::Fp;
use crate::recursion_assembly::compose_step::compose_step_region;
use crate::recursion_assembly::inner::{hasher, step_air};

#[test]
fn step_compose_region_accepts_the_honest_composition() {
    let h = hasher();
    let (region, trace) = compose_step_region(step_air(&h));
    let out = region.transition(&trace, &[]);
    assert_eq!(out.len(), region.num_transition(), "constraint count");
    assert!(
        out.iter().all(|v| *v == Fp::ZERO),
        "the honest step composition must satisfy every constraint"
    );
}

#[test]
fn step_compose_accessors_match_the_layout() {
    let h = hasher();
    let (region, trace) = compose_step_region(step_air(&h));
    let w = region.trace_width();

    // Every accessor is a c0-lane column: even, and its c1 lane is in bounds.
    for col in [region.z_col(), region.comp_z_col(), region.coeff_col(0), region.periodic_col(0)] {
        assert!(col % 2 == 0 && col + 1 < w, "accessor column out of layout");
    }
    // The frame occupies the first slots, so its base column is 2*i.
    assert_eq!(region.frame_col(0), 0);
    assert_eq!(region.frame_col(1), 2);
    // Slot order: frame, then periodic, then z, then coefficients.
    assert!(region.frame_col(region.frame_len() - 1) < region.periodic_col(0));
    assert!(region.periodic_col(0) < region.z_col());
    assert!(region.z_col() < region.coeff_col(0));
    // The point cell the accessor names is the one the trace actually populated.
    assert!(
        trace[region.z_col()] != Fp::ZERO || trace[region.z_col() + 1] != Fp::ZERO,
        "z_col does not land on the populated point cell"
    );
}

#[test]
fn step_compose_region_rejects_a_tampered_frame() {
    let h = hasher();
    let (region, trace) = compose_step_region(step_air(&h));
    let mut bad = trace.clone();
    bad[0] = bad[0] + Fp::ONE;
    let out = region.transition(&bad, &[]);
    assert!(
        out.iter().any(|v| *v != Fp::ZERO),
        "a tampered frame cell must break the recompute or the composition binding"
    );
}
