// NONOS Operating System (AGPL-3.0-or-later)

use super::fold::Tree;
use super::hash::{empty_leaf, leaf_hash};
use super::insert::{chain, writes_are_distinct, Low};
use super::last::last_is_the_maximum;
use super::leaf::Leaf;
use super::order::cmp;
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// The nullifier set, small enough to hold whole, for arguing about what a batch
/// does to its root.
pub struct Set {
    pub leaves: Vec<Leaf>,
    pub slots: usize,
}

impl Set {
    pub fn genesis(slots: usize) -> Set {
        Set {
            leaves: alloc::vec![Leaf::sentinel()],
            slots,
        }
    }

    pub fn root(&self, h: &Poseidon) -> [Fp; RATE] {
        let mut cells: Vec<[Fp; RATE]> = self.leaves.iter().map(|l| leaf_hash(h, l)).collect();
        cells.resize(self.slots, empty_leaf(h));
        Tree::build(h, cells).root()
    }

    /// Insert a sorted batch, or refuse. Refusing is the point: a key already in
    /// the set has no gap, so a transition claiming to insert it is one the tree
    /// cannot realise however happily the state carry composed it.
    pub fn insert(&self, keys: &[[Fp; RATE]]) -> Option<Set> {
        if self.leaves.len() + keys.len() > self.slots {
            return None;
        }
        let steps = chain(keys, &self.leaves)?;
        if !writes_are_distinct(&steps) {
            return None;
        }
        let mut next = self.leaves.clone();
        for (i, s) in steps.iter().enumerate() {
            let at = match s.low {
                Low::InTree(j) => j,
                Low::InBatch(j) => next
                    .iter()
                    .position(|l| cmp(&l.value, &keys[j]) == Ordering::Equal)?,
            };
            // The new leaf takes what the low one pointed at, and the low one
            // points at the new leaf. Two writes, one each, which is what
            // distinctness bought.
            let (took, was_last) = (next[at].next_value, next[at].is_last);
            next[at].next_value = s.key;
            next[at].is_last = false;
            next.push(Leaf {
                value: s.key,
                next_index: i as u64,
                next_value: took,
                is_last: was_last,
            });
            next.sort_by(|a, b| cmp(&a.value, &b.value));
        }
        last_is_the_maximum(&next).then_some(Set {
            leaves: next,
            slots: self.slots,
        })
    }
}
