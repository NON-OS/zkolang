// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

pub(crate) const TREE_DEPTH: usize = 32;

/// GoldilocksIncrementalTree: zeros[0] = 0, zeros[i+1] = hash2(zeros[i], zeros[i]),
/// frontier insert taking the node left when the index bit is zero. Leaves are kept
/// so a path can be produced for a past leaf, which the frontier alone cannot do.
pub(crate) struct PoolTree {
    h: Poseidon,
    zeros: Vec<[Fp; RATE]>,
    leaves: Vec<[Fp; RATE]>,
}

impl PoolTree {
    pub fn new(h: Poseidon) -> PoolTree {
        let mut zeros = Vec::with_capacity(TREE_DEPTH + 1);
        let mut z = [Fp::ZERO; RATE];
        for _ in 0..=TREE_DEPTH {
            zeros.push(z);
            z = h.compress(&z, &z);
        }
        PoolTree { h, zeros, leaves: Vec::new() }
    }

    pub fn insert(&mut self, leaf: [Fp; RATE]) -> usize {
        self.leaves.push(leaf);
        self.leaves.len() - 1
    }

    fn node(&self, level: usize, idx: usize) -> [Fp; RATE] {
        if level == 0 {
            return *self.leaves.get(idx).unwrap_or(&self.zeros[0]);
        }
        if idx * (1usize << level) >= self.leaves.len() {
            return self.zeros[level];
        }
        let l = self.node(level - 1, idx * 2);
        let r = self.node(level - 1, idx * 2 + 1);
        self.h.compress(&l, &r)
    }

    pub fn root(&self) -> [Fp; RATE] {
        self.node(TREE_DEPTH, 0)
    }

    pub fn path(&self, index: usize) -> (Vec<[Fp; RATE]>, Vec<bool>) {
        let mut sibs = Vec::with_capacity(TREE_DEPTH);
        let mut dirs = Vec::with_capacity(TREE_DEPTH);
        let mut idx = index;
        for level in 0..TREE_DEPTH {
            sibs.push(self.node(level, idx ^ 1));
            dirs.push(idx & 1 == 1);
            idx >>= 1;
        }
        (sibs, dirs)
    }
}
