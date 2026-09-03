// NONOS Operating System (AGPL-3.0-or-later)
//! Region 2: the DEEP consistency check for one query in witness form. The terms
//! and the evaluation point ride the trace; the tampers that must be rejected are
//! a trace value cut loose from its authenticated opening and a batching
//! coefficient off the transcript, both internally consistent in-region so only
//! the binding catches them.

use super::inner::{extra, Inner};
use super::tamper::Tamper;
use crate::crypto::stark::air::{
    deep_terms_pre_queryk, deep_terms_queryk_pub, AirExt, DeepCheckExt, Poseidon,
};
use crate::crypto::stark::field::{Fp, Fp2};
use alloc::vec::Vec;

/// Query-0 form, preserved for the single-query assembly.
pub fn deep_region<A: AirExt>(
    h: &Poseidon,
    inner: &Inner<A>,
    tamper: Tamper,
) -> (DeepCheckExt, Vec<Fp>, usize) {
    deep_region_k(h, inner, 0, tamper)
}

pub fn deep_region_k<A: AirExt>(
    h: &Poseidon,
    inner: &Inner<A>,
    query: usize,
    tamper: Tamper,
) -> (DeepCheckExt, Vec<Fp>, usize) {
    // A preprocessed inner replays the sidecar transcript and gains one
    // quotient per periodic column; the plain path is unchanged.
    let (mut terms, dx, ddeep) = match &inner.sidecar {
        Some(sc) => deep_terms_pre_queryk(
            &inner.air,
            &inner.proof,
            &sc.periodic_z,
            &sc.openings[query].row,
            extra(),
            h,
            &inner.publics,
            query,
        ),
        None => deep_terms_queryk_pub(&inner.air, &inner.proof, extra(), h, &inner.publics, query),
    };
    match tamper {
        Tamper::ReboundTraceValue => terms[0].val = terms[0].val + Fp2::ONE,
        Tamper::OffTranscriptCoeff => terms[0].coeff = terms[0].coeff + Fp2::ONE,
        _ => {}
    }
    let n_terms = terms.len();
    // The comp term sits after the frame terms; behind it the sidecar's
    // periodic terms, so last() is not it.
    let n_extra = inner.sidecar.as_ref().map(|sc| sc.periodic_z.len()).unwrap_or(0);
    let comp_index = n_terms - 1 - n_extra;
    let region = DeepCheckExt::new_witness_with_comp(terms, dx, ddeep, comp_index);
    let trace = region.trace();
    (region, trace, n_terms)
}
