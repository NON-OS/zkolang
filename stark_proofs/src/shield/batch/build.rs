// NONOS Operating System (AGPL-3.0-or-later)

use super::uniform::price_uniform;
use crate::crypto::stark::air::{Air, AirExt, Publics, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use crate::shield::wire::offsets;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Batch {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
}

/// The batch public surface: one intent tuple per region, with the clearing
/// price tied across all of them. The per intent circuits stack beneath these
/// and bind their own derived words.
pub(crate) fn batch(tuples: &[Vec<Fp>]) -> Batch {
    let mut regions: Vec<Box<dyn AirExt>> = Vec::with_capacity(tuples.len());
    let mut traces: Vec<Vec<Fp>> = Vec::with_capacity(tuples.len());
    for t in tuples {
        let air = Publics { log_t: 5, words: t.clone() };
        traces.push(air.trace());
        regions.push(Box::new(air));
    }
    let rows: Vec<usize> = regions.iter().map(|r| 1usize << r.log_trace_len()).collect();
    let (off, span) = offsets(&rows);
    let wired = WiredMultiExt::new(regions, price_uniform(span, &off));
    let witness = wired.trace(&traces);
    Batch { wired, witness }
}
