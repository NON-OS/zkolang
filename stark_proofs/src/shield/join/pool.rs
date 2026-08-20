// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{AirExt, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::key::Break;
use crate::shield::member::{note_member, PoolTree};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Pool {
    pub regions: Vec<Box<dyn AirExt>>,
    pub traces: Vec<Vec<Fp>>,
    pub leaf_col: Vec<usize>,
    pub root: [Fp; RATE],
    pub leaves: Vec<usize>,
}

pub(crate) fn pool_membership(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    depth: usize,
    brk: Break,
) -> Pool {
    let mut tree = PoolTree::with_depth(h.clone(), depth);
    let leaves: Vec<usize> = cms.iter().map(|cm| tree.insert(*cm)).collect();
    let mut regions: Vec<Box<dyn AirExt>> = Vec::with_capacity(2);
    let mut traces = Vec::with_capacity(2);
    let mut leaf_col = Vec::with_capacity(2);
    for (i, cm) in cms.iter().enumerate() {
        // The second note walks a pool of its own. It sits at the same position it
        // holds in the real tree, so its path directions are the ones the position
        // binding expects and only the siblings, and so the root, differ. That
        // leaves exactly one tie to violate: the walked root against the published
        // one, which for this note is tied to nothing.
        let mut side = PoolTree::with_depth(h.clone(), depth);
        let (sibs, dirs, root) = if brk == Break::ForeignPoolRoot && i == 1 {
            for _ in 0..leaves[i] {
                side.insert([Fp::from_u64(0xDEAD); RATE]);
            }
            let at = side.insert(*cm);
            let (s, d) = side.path(at);
            (s, d, side.root())
        } else {
            let (s, d) = tree.path(leaves[i]);
            (s, d, tree.root())
        };
        leaf_col.push(if dirs[0] { RATE } else { 0 });
        let m = note_member(h, *cm, sibs, dirs, root);
        regions.push(Box::new(m.region));
        traces.push(m.witness);
    }
    Pool { regions, traces, leaf_col, root: tree.root(), leaves }
}
