// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use super::assoc::assoc_membership;
use super::keys::key_hierarchies;
use crate::shield::key::Break;
use super::pool::pool_membership;
use crate::shield::note::{note_parts, Note};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Stack {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    pub span_op: usize,
    pub leaf_col: Vec<usize>,
    pub key_span: Vec<usize>,
    pub root: [Fp; RATE],
    pub nf: [[Fp; RATE]; 2],
    pub out_cm: [[Fp; RATE]; 2],
    pub assoc_root: [Fp; RATE],
    pub assoc_col: Vec<usize>,
}

/// Region order: balance, four notes, two memberships, two key hierarchies. The
/// bindings address regions by that order.
pub(crate) fn stack(
    h: &Poseidon,
    notes: [&Note; 4],
    sks: [[Fp; RATE]; 2],
    brk: Break,
    bal: (Box<dyn AirExt>, Vec<Fp>),
) -> Stack {
    let parts: Vec<_> = notes.iter().map(|n| note_parts(n)).collect();
    let span_op = parts[0].span_op;
    let cms: Vec<[Fp; RATE]> = parts.iter().map(|p| p.cm).collect();

    let mut regions: Vec<Box<dyn AirExt>> = alloc::vec![bal.0];
    let mut traces = alloc::vec![bal.1];
    for p in parts {
        regions.push(Box::new(p.region));
        traces.push(p.trace);
    }

    let p = pool_membership(h, &[cms[0], cms[1]]);
    let leaves = p.leaves.clone();
    let leaf_col = p.leaf_col.clone();
    regions.extend(p.regions);
    traces.extend(p.traces);

    let k = key_hierarchies(sks, &[cms[0], cms[1]], &leaves, brk);
    let key_span = k.key_span.clone();
    let nfs = k.nf;
    regions.extend(k.regions);
    traces.extend(k.traces);

    let a = assoc_membership(h, &[cms[0], cms[1]], &[900, 901, 902]);
    regions.extend(a.regions);
    traces.extend(a.traces);

    Stack {
        regions,
        traces,
        span_op,
        leaf_col,
        key_span,
        root: p.root,
        nf: [nfs[0], nfs[1]],
        out_cm: [cms[2], cms[3]],
        assoc_root: a.root,
        assoc_col: a.leaf_col,
    }
}
