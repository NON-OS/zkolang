// NONOS Operating System (AGPL-3.0-or-later)
//! Region 1 for a zkolang inner: the out-of-domain composition in generic form.
//! Where `compose_region` hand-reads the join-split frame, this hands the whole
//! frame to `ComposeCheckGen`, which recomputes the step AIR's transitions from
//! it via the AIR's own code at `Ext2<F>`. Nothing is transcribed by hand, so the
//! compose checks exactly what the step AIR proves. It takes the AIR by value: the
//! region owns it to recompute the transitions during proving.

use super::inner::Inner;
use crate::crypto::stark::air::{AirExt, ComposeCheckGen, GenericTransition};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;
use nonos_zkolang::StepAir;

pub fn compose_step_region(inner: Inner<StepAir>) -> (ComposeCheckGen<StepAir>, Vec<Fp>) {
    compose_gen_region(inner)
}

/// The same region for any inner whose transition the recursion can recompute
/// over the tower. The step AIR and the deployed join-split both come through
/// here, so the generic compose cannot fork per inner.
pub fn compose_gen_region<A: AirExt + GenericTransition>(
    inner: Inner<A>,
) -> (ComposeCheckGen<A>, Vec<Fp>) {
    let region = ComposeCheckGen::new_witness(
        inner.air,
        inner.proof.ood_frame,
        inner.ci.periodic_z,
        inner.ci.coeffs,
        inner.ci.z,
        inner.ci.comp_z,
        inner.g,
    );
    let trace = region.trace();
    (region, trace)
}
