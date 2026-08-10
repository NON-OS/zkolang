// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::{nullifier_parts, Break};
use crate::shield::member::{note_member, PoolTree};
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

    let mut tree = PoolTree::new(h.clone());
    let leaves: Vec<usize> = (0..2).map(|i| tree.insert(cms[i])).collect();

    let mut leaf_col = Vec::with_capacity(2);
    for i in 0..2 {
        let (sibs, dirs) = tree.path(leaves[i]);
        leaf_col.push(if dirs[0] { RATE } else { 0 });
        let m = note_member(h, cms[i], sibs, dirs, tree.root());
        regions.push(Box::new(m.region));
        traces.push(m.witness);
    }

    let mut key_span = Vec::with_capacity(2);
    let mut nfs = alloc::vec![[Fp::ZERO; RATE]; 2];
    for i in 0..2 {
        let k = nullifier_parts(sks[i], cms[i], leaves[i] as u64, brk);
        key_span.push(k.span_op);
        nfs[i] = k.nf;
        regions.push(Box::new(k.region));
        traces.push(k.trace);
    }

    Stack {
        regions,
        traces,
        span_op,
        leaf_col,
        key_span,
        root: tree.root(),
        nf: [nfs[0], nfs[1]],
        out_cm: [cms[2], cms[3]],
    }
}
