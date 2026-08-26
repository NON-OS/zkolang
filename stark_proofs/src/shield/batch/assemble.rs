// NONOS Operating System (AGPL-3.0-or-later)

use super::uniform::price_uniform;
use crate::crypto::stark::air::{Air, AirExt, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use crate::shield::join::{
    bind_classes, public_classes_at, IntentParts, Layout, REGIONS_PER_INTENT,
};
use crate::crypto::stark::air::classes_are_disjoint;
use crate::shield::wire::offsets;
use crate::shield::wire_pack::{packed_groups, CAP};
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
    let meta: Vec<(usize, Vec<usize>, Vec<usize>, Vec<usize>, usize)> = parts
        .into_iter()
        .map(|p| {
            intents.push(p.intent);
            regions.extend(p.regions);
            traces.extend(p.traces);
            (p.span_op, p.leaf_col, p.key_span, p.assoc_col, p.depth)
        })
        .collect();

    let rows: Vec<usize> = regions.iter().map(|r| r.rows()).collect();
    let (off, span) = offsets(&rows);

    let mut g: Vec<crate::shield::wire_class::Class> = Vec::new();
    let mut pub_off = Vec::with_capacity(meta.len());
    for (i, (span_op, leaf_col, key_span, assoc_col, depth)) in meta.into_iter().enumerate() {
        let b = i * REGIONS_PER_INTENT;
        let lay = Layout {
            span,
            span_op,
            note: off[b + 1..b + 5].to_vec(),
            member: off[b + 5..b + 7].to_vec(),
            index: off[b + 7..b + 9].to_vec(),
            key: off[b + 9..b + 11].to_vec(),
            key_span,
            leaf_col,
            assoc: off[b + 11..b + 13].to_vec(),
            assoc_col,
            depth,
            balance: off[b],
        };
        g.extend(bind_classes(&lay));
        g.extend(public_classes_at(&lay, off[b + 13]));
        pub_off.push(off[b + 13]);
    }
    g.extend(price_uniform(&pub_off));

    // Classes are the bindings; one group each is how they are enforced today.
    // Disjointness is a precondition of merging them, so it is proven before the
    // mechanism is allowed to assume it.
    debug_assert!(classes_are_disjoint(&g), "binding classes overlap");
    // Regions stack vertically and share columns, so the addressable width is the
    // widest region, not the sum.
    // An intent lays out balance, four note commitments, two pool memberships, two
    // position recoveries, two key derivations, two association memberships, then
    // its publics. The four and the pairs are one AIR over different witnesses, and
    // every intent repeats the same fourteen, so the whole batch carries seven kinds.
    let kinds: Vec<usize> = (0..regions.len() / REGIONS_PER_INTENT)
        .flat_map(|_| alloc::vec![0usize, 1, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6])
        .collect();
    let groups = packed_groups(span, &g, CAP);
    let wired = WiredMultiExt::new_kinds(regions, &kinds, groups);
    let witness = wired.trace(&traces);
    BatchProof { wired, witness, intents }
}
