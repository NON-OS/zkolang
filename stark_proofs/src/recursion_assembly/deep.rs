// NONOS Operating System (AGPL-3.0-or-later)
//! Region 2: the DEEP consistency check for one query in witness form. The terms
//! and the evaluation point ride the trace; the tampers that must be rejected are
//! a trace value cut loose from its authenticated opening and a batching
//! coefficient off the transcript, both internally consistent in-region so only
//! the binding catches them.

use super::inner::{Inner, EXTRA};
use super::tamper::Tamper;
use crate::crypto::stark::air::{deep_terms_queryk_pub, AirExt, DeepCheckExt, Poseidon};
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
    let (mut terms, dx, ddeep) =
        deep_terms_queryk_pub(&inner.air, &inner.proof, EXTRA, h, &inner.publics, query);
    match tamper {
        Tamper::ReboundTraceValue => terms[0].val = terms[0].val + Fp2::ONE,
        Tamper::OffTranscriptCoeff => terms[0].coeff = terms[0].coeff + Fp2::ONE,
        _ => {}
    }
    let n_terms = terms.len();
    let region = DeepCheckExt::new_witness(terms, dx, ddeep);
    let trace = region.trace();
    (region, trace, n_terms)
}
