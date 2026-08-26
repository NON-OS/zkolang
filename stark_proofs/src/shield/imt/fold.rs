// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// A tree small enough to hold whole, for arguing about what a batch update does.
pub struct Tree {
    pub depth: usize,
    /// Level 0 is the leaves, level `depth` is the root.
    pub level: Vec<Vec<[Fp; RATE]>>,
}

impl Tree {
    pub fn build(h: &Poseidon, leaves: Vec<[Fp; RATE]>) -> Tree {
        let depth = leaves.len().trailing_zeros() as usize;
        let mut level = alloc::vec![leaves];
        for d in 0..depth {
            let below = &level[d];
            let up = (0..below.len() / 2)
                .map(|i| h.compress(&below[2 * i], &below[2 * i + 1]))
                .collect();
            level.push(up);
        }
        Tree { depth, level }
    }

    pub fn root(&self) -> [Fp; RATE] {
        self.level[self.depth][0]
    }
}

/// The root after changing a set of leaves, recomputing each internal node once
/// from whichever of its children moved.
///
/// Written as "combine the changed pairs" this would handle two siblings and fail
/// on a cluster of four under one small subtree, which is the same
/// sufficient-not-necessary trap one level down. Recomputing a node once from its
/// possibly-updated children takes any density without knowing the density.
///
/// Keyed by index, so the order the changes arrive in cannot reach the root. If it
/// could, the shape of the aggregation tree would leak into the state and two
/// valid orders would disagree.
pub fn refold(h: &Poseidon, tree: &Tree, changed: &[(usize, [Fp; RATE])]) -> [Fp; RATE] {
    let mut moved: BTreeMap<usize, [Fp; RATE]> = changed.iter().copied().collect();
    for d in 0..tree.depth {
        let mut up: BTreeMap<usize, [Fp; RATE]> = BTreeMap::new();
        for i in moved.keys() {
            let p = i / 2;
            if up.contains_key(&p) {
                continue;
            }
            let at = |j: usize| *moved.get(&j).unwrap_or(&tree.level[d][j]);
            up.insert(p, h.compress(&at(2 * p), &at(2 * p + 1)));
        }
        moved = up;
    }
    moved.remove(&0).unwrap_or_else(|| tree.root())
}
