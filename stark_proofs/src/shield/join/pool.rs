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

/// One input's opening against a tree somebody else owns: where the note sits
/// and the siblings on the way up.
///
/// Directions are not carried. A caller that passes both a position and a
/// direction list can pass two descriptions of one path, and the pair that
/// disagrees is a note retired under a position the pool never authenticated.
/// The position is the single source and the directions come off its bits
/// here, so the disagreement cannot be expressed.
pub struct Witnessed {
    pub leaf_index: usize,
    pub siblings: Vec<[Fp; RATE]>,
}

impl Witnessed {
    /// Level `m` goes right when bit `m` of the position is set, which is the
    /// same convention `PoolTree::path` returns and the same one the index
    /// binding recovers the scalar from.
    fn directions(&self) -> Vec<bool> {
        (0..self.siblings.len()).map(|m| (self.leaf_index >> m) & 1 == 1).collect()
    }
}

/// Membership against a tree this builder owns: it plants one, inserts the two
/// commitments and proves against the root it just computed.
///
/// That is a closed loop. The statement it proves is that the notes belong to a
/// tree containing exactly those notes, under a root the prover chose, which no
/// pool ever published. It is the right shape for a test and the wrong shape
/// for a deployment, so it stays for the fixtures and
/// [`pool_membership_against`] is what a real spend uses.
pub fn pool_membership(h: &Poseidon, cms: &[[Fp; RATE]; 2], depth: usize) -> Pool {
    let mut tree = PoolTree::with_depth(h.clone(), depth);
    let leaves: Vec<usize> = cms.iter().map(|cm| tree.insert(*cm)).collect();
    let openings: Vec<Witnessed> = leaves
        .iter()
        .map(|&i| Witnessed { leaf_index: i, siblings: tree.path(i).0 })
        .collect();
    build(h, cms, &[&openings[0], &openings[1]], tree.root())
}

/// Membership against a root the pool published.
///
/// The caller hands the openings and the root; nothing here plants a tree. The
/// constraints still do not enforce membership, because a tampered path is
/// honest arithmetic that walks somewhere else, so membership remains the
/// walked root equalling the published one. What changes is which root that is:
/// a value the verifier already holds rather than one arriving with the proof.
pub fn pool_membership_against(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    openings: [&Witnessed; 2],
    root: [Fp; RATE],
) -> Pool {
    build(h, cms, &openings, root)
}

fn build(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    openings: &[&Witnessed; 2],
    root: [Fp; RATE],
) -> Pool {
    let mut regions: Vec<ShieldRegion> = Vec::with_capacity(2);
    let mut traces = Vec::with_capacity(2);
    let mut leaf_col = Vec::with_capacity(2);
    let mut leaves = Vec::with_capacity(2);
    for (i, cm) in cms.iter().enumerate() {
        let o = openings[i];
        let m = note_member(h, *cm, o.siblings.clone(), o.directions(), root);
        // The membership pins the bottom bit, so the note commitment binds to the
        // canonical leaf, whose column is fixed rather than chosen by the bottom
        // direction. The select constraint ties that leaf back to the real half.
        leaf_col.push(m.region.leaf_col());
        regions.push(ShieldRegion::Membership(m.region));
        traces.push(m.witness);
        leaves.push(o.leaf_index);
    }
    Pool { regions, traces, leaf_col, root, leaves }
}
