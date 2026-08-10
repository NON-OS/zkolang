// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::member::{note_member, PoolTree};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Assoc {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    pub leaf_col: Vec<usize>,
    pub root: [Fp; RATE],
}

/// The registry is permissionless and only checks that a root is registered, so
/// membership in the set is the circuit's job. Same construction as the pool
/// tree: a spend that cannot open against a published set has no proof.
pub(crate) fn assoc_membership(h: &Poseidon, cms: &[[Fp; RATE]; 2], others: &[u64]) -> Assoc {
    let mut tree = PoolTree::new(h.clone());
    for s in others {
        let mut pad = [Fp::ZERO; RATE];
        pad[0] = Fp::from_u64(*s);
        tree.insert(pad);
    }
    let idx: Vec<usize> = cms.iter().map(|cm| tree.insert(*cm)).collect();

    let mut regions: Vec<Box<dyn AirExt>> = Vec::with_capacity(2);
    let mut traces = Vec::with_capacity(2);
    let mut leaf_col = Vec::with_capacity(2);
    for (i, cm) in cms.iter().enumerate() {
        let (sibs, dirs) = tree.path(idx[i]);
        leaf_col.push(if dirs[0] { RATE } else { 0 });
        let m = note_member(h, *cm, sibs, dirs, tree.root());
        regions.push(Box::new(m.region));
        traces.push(m.witness);
    }
    Assoc { regions, traces, leaf_col, root: tree.root() }
}
