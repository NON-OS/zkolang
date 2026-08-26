// NONOS Operating System (AGPL-3.0-or-later)

use super::leaf::Leaf;
use super::order::cmp;
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;
use core::cmp::Ordering;

/// Where a key enters the chain: either between two leaves already there, or
/// after the value the batch inserted just before it.
pub enum Low {
    InTree(usize),
    InBatch(usize),
}

pub struct Step {
    pub key: [Fp; RATE],
    pub low: Low,
}

/// No two keys update the same leaf.
///
/// This holds today because a same-gap key takes the key before it as its low
/// leaf rather than the leaf they both sit under. That is a consequence, not a
/// property: validate every key against the pre-batch tree instead and both
/// same-gap keys satisfy `L.value < key < L.nextValue`, both write `L.next`, they
/// collide, and the fold loses one without saying so. Separate gaps stay green
/// throughout, which is what makes it a refactor away.
///
/// So the circuit checks it rather than inheriting it.
pub fn writes_are_distinct(steps: &[Step]) -> bool {
    let mut seen: Vec<&Low> = Vec::with_capacity(steps.len());
    for s in steps {
        let clash = seen.iter().any(|o| match (o, &s.low) {
            (Low::InTree(a), Low::InTree(b)) => a == b,
            (Low::InBatch(a), Low::InBatch(b)) => a == b,
            _ => false,
        });
        if clash {
            return false;
        }
        seen.push(&s.low);
    }
    true
}

/// A batch, sorted, so sort-adjacent keys do not fight over one neighbour.
///
/// An indexed tree insert mutates the low leaf's pointer, so two new keys landing
/// beside each other touch the same leaf and serialise. Sorting first turns the
/// batch into a chain: each key's low leaf is either an existing leaf or the key
/// before it, and the whole run proves as one subtree.
///
/// Strictly increasing, so a duplicate cannot survive the chain. Uniqueness stops
/// being a rule to enforce and becomes a consequence of the shape, which is one
/// fewer place to be subtly wrong.
pub fn chain(sorted: &[[Fp; RATE]], tree: &[Leaf]) -> Option<Vec<Step>> {
    for w in sorted.windows(2) {
        if cmp(&w[0], &w[1]) != Ordering::Less {
            return None;
        }
    }
    let mut steps = Vec::with_capacity(sorted.len());
    for (i, key) in sorted.iter().enumerate() {
        let in_tree = tree
            .iter()
            .enumerate()
            .filter(|(_, l)| cmp(&l.value, key) == Ordering::Less)
            .max_by(|(_, a), (_, b)| cmp(&a.value, &b.value))
            .map(|(j, _)| j)?;
        // The preceding batch key outranks the tree's low leaf whenever it is
        // larger, which after sorting it always is once there is one.
        steps.push(Step {
            key: *key,
            low: if i > 0 && cmp(&tree[in_tree].value, &sorted[i - 1]) == Ordering::Less {
                Low::InBatch(i - 1)
            } else {
                Low::InTree(in_tree)
            },
        });
    }
    Some(steps)
}
