// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

/// GoldilocksIncrementalTree.TREE_DEPTH.
pub(crate) const TREE_DEPTH: usize = 32;

/// zeros[0] = 0, zeros[i+1] = hash2(zeros[i], zeros[i]), frontier insert taking
/// the node left when the index bit is zero. Leaves are kept so a path can be
/// produced for a past leaf, which the frontier alone cannot do.
///
/// Depth is a field so a forgery can run on a minimal instance: violating a
/// binding rejects through the permutation structure, not the tree size.
pub(crate) struct PoolTree {
    h: Poseidon,
    depth: usize,
    zeros: Vec<[Fp; RATE]>,
    leaves: Vec<[Fp; RATE]>,
}

impl PoolTree {
    pub fn new(h: Poseidon) -> PoolTree {
        Self::with_depth(h, TREE_DEPTH)
    }

    pub fn with_depth(h: Poseidon, depth: usize) -> PoolTree {
        let mut zeros = Vec::with_capacity(depth + 1);
        let mut z = [Fp::ZERO; RATE];
        for _ in 0..=depth {
            zeros.push(z);
            z = h.compress(&z, &z);
        }
        PoolTree {
            h,
            depth,
            zeros,
            leaves: Vec::new(),
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
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
        self.node(self.depth, 0)
    }

    pub fn path(&self, index: usize) -> (Vec<[Fp; RATE]>, Vec<bool>) {
        let mut sibs = Vec::with_capacity(self.depth);
        let mut dirs = Vec::with_capacity(self.depth);
        let mut idx = index;
        for level in 0..self.depth {
            sibs.push(self.node(level, idx ^ 1));
            dirs.push(idx & 1 == 1);
            idx >>= 1;
        }
        (sibs, dirs)
    }
}
