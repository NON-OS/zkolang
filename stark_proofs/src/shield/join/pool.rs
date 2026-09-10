// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, ShieldRegion, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::member::{note_member, PoolTree};
use alloc::vec::Vec;

pub struct Pool {
    pub regions: Vec<ShieldRegion>,
    pub traces: Vec<Vec<Fp>>,
    pub leaf_col: Vec<usize>,
    pub root: [Fp; RATE],
    pub leaves: Vec<usize>,
}

pub fn pool_membership(h: &Poseidon, cms: &[[Fp; RATE]; 2], depth: usize) -> Pool {
    let mut tree = PoolTree::with_depth(h.clone(), depth);
    let leaves: Vec<usize> = cms.iter().map(|cm| tree.insert(*cm)).collect();
    let mut regions: Vec<ShieldRegion> = Vec::with_capacity(2);
    let mut traces = Vec::with_capacity(2);
    let mut leaf_col = Vec::with_capacity(2);
    for (i, cm) in cms.iter().enumerate() {
        let (sibs, dirs) = tree.path(leaves[i]);
        let m = note_member(h, *cm, sibs, dirs, tree.root());
        // The membership pins the bottom bit, so the note commitment binds to the
        // canonical leaf, whose column is fixed rather than chosen by the bottom
        // direction. The select constraint ties that leaf back to the real half.
        leaf_col.push(m.region.leaf_col());
        regions.push(ShieldRegion::Membership(m.region));
        traces.push(m.witness);
    }
    Pool { regions, traces, leaf_col, root: tree.root(), leaves }
}
