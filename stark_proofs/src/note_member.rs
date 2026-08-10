// NONOS Operating System (AGPL-3.0-or-later)
//! Note membership: prove a note commitment sits at a leaf of the pool's tree
//! under a published root, without revealing which leaf.
//!
//! `GoldilocksIncrementalTree` is a depth-32 Poseidon tree with
//! `zeros[0] = 0`, `zeros[i+1] = hash2(zeros[i], zeros[i])`, and a frontier
//! insert that takes the running node as the LEFT child when the index bit is
//! zero. `hash2` is the pool's 32-round `compress`, the same primitive
//! `commit_note` is built from and the same one `PoseidonGoldilocks.sol` is
//! KAT-gated against.
//!
//! That is exactly a `MultiMembership` opening: `inject(node, sibling, right)`
//! places the node low and the sibling high when `right` is false, so a path
//! whose directions are the index bits reproduces the contract's insert
//! arithmetic level for level. The circuit therefore does not re-implement the
//! tree, it replays it.
//!
//! Scope: this proves `cm` is IN the tree. It deliberately does not touch the
//! nullifier or the spending key, which need the key hierarchy pinned in
//! SPEC.md §4. The join-split assembly is where this cm gets bound to the
//! nullifier's cm and to the `noteRoot` public input; on its own this region
//! proves membership and nothing more.

use crate::crypto::stark::air::{
    Air, MultiMembership, Opening, Poseidon, RATE,
};
use crate::crypto::stark::field::Fp;
use alloc::vec::Vec;

use crate::note_commit::POOL_LOG_ROUNDS;

/// The deployed tree depth (`GoldilocksIncrementalTree.TREE_DEPTH`).
pub(crate) const TREE_DEPTH: usize = 32;

/// The pool tree, replaying the contract's construction so a proof is against
/// the same arithmetic the chain performs. Leaves are kept so an authentication
/// path can be produced for any past leaf, which the frontier alone cannot do.
pub(crate) struct PoolTree {
    h: Poseidon,
    zeros: Vec<[Fp; RATE]>,
    leaves: Vec<[Fp; RATE]>,
}

impl PoolTree {
    pub fn new(h: Poseidon) -> PoolTree {
        // zeros[0] = the canonical empty leaf, then doubled up the levels.
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

    /// The node at `(level, idx)`, empty subtrees collapsing to `zeros[level]`
    /// exactly as the contract's insert does.
    fn node(&self, level: usize, idx: usize) -> [Fp; RATE] {
        if level == 0 {
            return *self.leaves.get(idx).unwrap_or(&self.zeros[0]);
        }
        let span = 1usize << level;
        if idx * span >= self.leaves.len() {
            return self.zeros[level];
        }
        let l = self.node(level - 1, idx * 2);
        let r = self.node(level - 1, idx * 2 + 1);
        self.h.compress(&l, &r)
    }

    pub fn root(&self) -> [Fp; RATE] {
        self.node(TREE_DEPTH, 0)
    }

    /// The authentication path for `index`: the sibling at every level, and the
    /// direction bits the contract's insert uses.
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

pub(crate) struct NoteMember {
    pub region: MultiMembership,
    pub witness: Vec<Fp>,
    /// The root the trace actually walks to, read back out of the trace rather
    /// than taken on trust.
    pub proven_root: [Fp; RATE],
}

/// Prove `leaf` opens to a root at `index` along `sibs`/`dirs`.
pub(crate) fn note_member(
    h: &Poseidon,
    leaf: [Fp; RATE],
    sibs: Vec<[Fp; RATE]>,
    dirs: Vec<bool>,
    root: [Fp; RATE],
) -> NoteMember {
    let opening = Opening { leaf, root, siblings: sibs, directions: dirs };
    let region = MultiMembership::new_witness(h.clone(), POOL_LOG_ROUNDS, alloc::vec![opening]);
    let witness = region.trace();

    // The walked root lands in the checkpoint slot, one slot past the last
    // compression: row depth * rounds, low lanes.
    let w = region.trace_width();
    let row = TREE_DEPTH * (1usize << POOL_LOG_ROUNDS);
    let mut proven_root = [Fp::ZERO; RATE];
    proven_root.copy_from_slice(&witness[row * w..row * w + RATE]);

    NoteMember { region, witness, proven_root }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note_commit::{note_commit, Note};

    fn hasher() -> Poseidon {
        Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
    }

    fn note(seed: u64) -> Note {
        Note {
            value: 1_000 + seed,
            asset_id: 0,
            spend_pk: [seed + 1, seed + 2, seed + 3, seed + 4],
            blinding: [seed + 5, seed + 6, seed + 7, seed + 8],
        }
    }

    fn satisfies(air: &MultiMembership, witness: &[Fp]) -> bool {
        let w = air.trace_width();
        let ws = air.window_size();
        let total = 1usize << air.log_trace_len();
        let periodic = air.periodic_columns();
        for r in 0..total - (ws - 1) {
            let mut window = Vec::with_capacity(ws * w);
            for k in 0..ws {
                window.extend_from_slice(&witness[(r + k) * w..(r + k + 1) * w]);
            }
            let per: Vec<Fp> = periodic.iter().map(|c| c[r]).collect();
            if air.transition(&window, &per).iter().any(|v| *v != Fp::ZERO) {
                return false;
            }
        }
        for (col, row, val) in air.boundary() {
            if witness[row * w + col] != val {
                return false;
            }
        }
        true
    }

    /// An empty pool's root is the zeros chain walked to the top, which is what
    /// the contract precomputes at deploy. If this drifts, the circuit is
    /// proving membership in a different tree than the chain keeps.
    #[test]
    fn empty_root_is_the_zeros_chain() {
        let h = hasher();
        let t = PoolTree::new(h.clone());
        let mut z = [Fp::ZERO; RATE];
        for _ in 0..TREE_DEPTH {
            z = h.compress(&z, &z);
        }
        assert_eq!(t.root(), z, "empty pool root != zeros chain");
    }

    /// THE bridge test: a real note commitment, inserted the way `deposit`
    /// inserts it, is provably in the tree under the root the pool publishes.
    #[test]
    fn a_deposited_note_is_provably_in_the_tree() {
        let h = hasher();
        let mut t = PoolTree::new(h.clone());
        // Several deposits, so the proven leaf is not the degenerate first one.
        let mut idx = 0;
        for s in 0..5u64 {
            let cm = note_commit(&note(s), false).cm;
            let i = t.insert(cm);
            if s == 3 {
                idx = i;
            }
        }
        let cm = note_commit(&note(3), false).cm;
        let (sibs, dirs) = t.path(idx);
        let m = note_member(&h, cm, sibs, dirs, t.root());

        assert!(satisfies(&m.region, &m.witness), "an honest membership must satisfy");
        assert_eq!(m.proven_root, t.root(), "the walked root is not the pool root");
    }

    /// Positive-gated negatives: each must reject, and the honest case above
    /// must still pass, so the rejection cannot be vacuous.
    #[test]
    fn a_wrong_sibling_does_not_reach_the_root() {
        let h = hasher();
        let mut t = PoolTree::new(h.clone());
        for s in 0..5u64 {
            t.insert(note_commit(&note(s), false).cm);
        }
        let cm = note_commit(&note(3), false).cm;
        let (mut sibs, dirs) = t.path(3);
        sibs[0][0] = sibs[0][0] + Fp::ONE;
        let m = note_member(&h, cm, sibs, dirs, t.root());
        // The path is internally consistent, so the constraints still hold; it
        // simply walks somewhere else. Membership is the root equality.
        assert!(satisfies(&m.region, &m.witness), "the trace itself stays honest");
        assert_ne!(m.proven_root, t.root(), "a tampered sibling still reached the root");
    }

    #[test]
    fn a_note_not_in_the_tree_does_not_reach_the_root() {
        let h = hasher();
        let mut t = PoolTree::new(h.clone());
        for s in 0..5u64 {
            t.insert(note_commit(&note(s), false).cm);
        }
        let outsider = note_commit(&note(99), false).cm;
        let (sibs, dirs) = t.path(3);
        let m = note_member(&h, outsider, sibs, dirs, t.root());
        assert_ne!(m.proven_root, t.root(), "a note never deposited reached the root");
    }

    /// The index is not free: opening the right leaf along the right siblings
    /// but claiming the wrong position walks elsewhere. This is what stops a
    /// spender from re-pointing a note at another leaf.
    #[test]
    fn the_wrong_index_does_not_reach_the_root() {
        let h = hasher();
        let mut t = PoolTree::new(h.clone());
        for s in 0..5u64 {
            t.insert(note_commit(&note(s), false).cm);
        }
        let cm = note_commit(&note(3), false).cm;
        let (sibs, mut dirs) = t.path(3);
        dirs[0] = !dirs[0];
        let m = note_member(&h, cm, sibs, dirs, t.root());
        assert_ne!(m.proven_root, t.root(), "a flipped index bit reached the root");
    }
}
