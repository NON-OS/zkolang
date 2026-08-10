// NONOS Operating System (AGPL-3.0-or-later)

use super::uniform::price_uniform;
use crate::crypto::stark::air::{Air, AirExt, GpGroup, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use crate::shield::join::{
    bind_groups, public_groups_at, IntentParts, Layout, REGIONS_PER_INTENT,
};
use crate::shield::wire::offsets;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct BatchProof {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub intents: Vec<Vec<Fp>>,
}

/// Every intent's regions in one stack, each intent's own bindings emitted at its
/// base, and the clearing price tied across all of them.
pub(crate) fn assemble(parts: Vec<IntentParts>) -> BatchProof {
    let mut regions: Vec<Box<dyn AirExt>> = Vec::new();
    let mut traces: Vec<Vec<Fp>> = Vec::new();
    let mut intents = Vec::with_capacity(parts.len());
    let meta: Vec<(usize, Vec<usize>, Vec<usize>)> = parts
        .into_iter()
        .map(|p| {
            intents.push(p.intent);
            regions.extend(p.regions);
            traces.extend(p.traces);
            (p.span_op, p.leaf_col, p.key_span)
        })
        .collect();

    let rows: Vec<usize> = regions.iter().map(|r| 1usize << r.log_trace_len()).collect();
    let (off, span) = offsets(&rows);

    let mut g: Vec<GpGroup> = Vec::new();
    let mut pub_off = Vec::with_capacity(meta.len());
    for (i, (span_op, leaf_col, key_span)) in meta.into_iter().enumerate() {
        let b = i * REGIONS_PER_INTENT;
        let lay = Layout {
            span,
            span_op,
            note: off[b + 1..b + 5].to_vec(),
            member: off[b + 5..b + 7].to_vec(),
            key: off[b + 7..b + 9].to_vec(),
            key_span,
            leaf_col,
            balance: off[b],
        };
        g.extend(bind_groups(&lay));
        g.extend(public_groups_at(&lay, off[b + 9]));
        pub_off.push(off[b + 9]);
    }
    g.extend(price_uniform(span, &pub_off));

    let wired = WiredMultiExt::new(regions, g);
    let witness = wired.trace(&traces);
    BatchProof { wired, witness, intents }
}
