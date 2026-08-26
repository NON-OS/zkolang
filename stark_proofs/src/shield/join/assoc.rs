// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::member::{note_member, PoolTree};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct Assoc {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    pub leaf_col: Vec<usize>,
    pub root: [Fp; RATE],
}

/// The registry is permissionless and only checks that a root is registered, so
/// membership in the set is the circuit's job. Same construction as the pool
/// tree: a spend that cannot open against a published set has no proof.
pub fn assoc_membership(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    others: &[u64],
    unlisted: bool,
    depth: usize,
) -> Assoc {
    let mut tree = PoolTree::with_depth(h.clone(), depth);
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
        // A note that really is in the set, but not the note being spent.
        let claimed = if unlisted { cms[1 - i] } else { *cm };
        let (sibs, dirs) = tree.path(if unlisted { idx[1 - i] } else { idx[i] });
        leaf_col.push(if dirs[0] { RATE } else { 0 });
        let m = note_member(h, claimed, sibs, dirs, tree.root());
        regions.push(Box::new(m.region));
        traces.push(m.witness);
    }
    Assoc { regions, traces, leaf_col, root: tree.root() }
}
