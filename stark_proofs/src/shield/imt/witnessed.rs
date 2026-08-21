// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use alloc::collections::BTreeMap;

/// One changed leaf and the siblings the prover supplies for its path.
pub(crate) struct Path {
    pub index: usize,
    pub leaf: [Fp; RATE],
    /// Sibling at each level, low to high.
    pub siblings: alloc::vec::Vec<[Fp; RATE]>,
}

/// The root, when the siblings are witness rather than a tree held whole.
///
/// The whole-tree fold gets single-valuedness for free: a node has one value
/// because it is read from the tree. In circuit each path carries its own
/// siblings, so two co-ancestral leaves can witness the ancestor they share with
/// two different values. Both then reach the root along their own path and the
/// root reflects neither.
///
/// So a node reached twice must be reached with the same value. That is the
/// in-circuit face of recomputing a node once, and nothing else replaces it: the
/// leaves can be distinct, every path can be internally consistent, and the root
/// is still a fiction.
pub(crate) fn root_of(h: &Poseidon, paths: &[Path]) -> Option<[Fp; RATE]> {
    // (level, index) -> the value some path computed there.
    let mut seen: BTreeMap<(usize, usize), [Fp; RATE]> = BTreeMap::new();
    let mut root: Option<[Fp; RATE]> = None;

    for p in paths {
        let (mut node, mut idx) = (p.leaf, p.index);
        let mut note = |lvl: usize, i: usize, v: [Fp; RATE], seen: &mut BTreeMap<_, _>| match seen
            .get(&(lvl, i))
        {
            Some(prev) if *prev != v => false,
            _ => {
                seen.insert((lvl, i), v);
                true
            }
        };
        if !note(0, idx, node, &mut seen) {
            return None;
        }
        for (lvl, sib) in p.siblings.iter().enumerate() {
            // The sibling is witnessed too, so it is a claim about a node and has
            // to agree with any other path that reached it.
            if !note(lvl, idx ^ 1, *sib, &mut seen) {
                return None;
            }
            node = if idx & 1 == 0 {
                h.compress(&node, sib)
            } else {
                h.compress(sib, &node)
            };
            idx >>= 1;
            if !note(lvl + 1, idx, node, &mut seen) {
                return None;
            }
        }
        match root {
            Some(r) if r != node => return None,
            _ => root = Some(node),
        }
    }
    root
}
