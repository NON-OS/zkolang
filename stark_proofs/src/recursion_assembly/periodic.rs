// NONOS Operating System (AGPL-3.0-or-later)
//! Region 8: the inner periodic columns at z, recomputed in-region with the
//! barycentric form over the inner trace domain, so the composition's
//! periodic inputs are derived from the bound z rather than trusted.

use super::inner::Inner;
use super::tamper::Tamper;
use crate::crypto::stark::air::{AirExt, PeriodicZ};
use crate::crypto::stark::field::{Fp, Fp2};
use alloc::vec::Vec;

pub fn periodic_region<A: AirExt>(
    inner: &Inner<A>,
    tamper: Tamper,
) -> (PeriodicZ, Vec<Fp>) {
    let z = match tamper {
        Tamper::PeriodicOffPoint => inner.ci.z + Fp2::ONE,
        _ => inner.ci.z,
    };
    let region = PeriodicZ::new(
        inner.air.log_trace_len(),
        inner.g,
        inner.air.periodic_columns(),
        z,
    );
    let trace = region.trace();
    (region, trace)
}
