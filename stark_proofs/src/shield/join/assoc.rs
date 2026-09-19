// NONOS Operating System (AGPL-3.0-or-later)

use super::pool::Witnessed;
use crate::crypto::stark::air::{Poseidon, ShieldRegion, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::member::{note_member, PoolTree};
use alloc::vec::Vec;

pub struct Assoc {
    pub regions: Vec<ShieldRegion>,
    pub traces: Vec<Vec<Fp>>,
    pub leaf_col: Vec<usize>,
    pub root: [Fp; RATE],
}

/// The registry is permissionless and only checks that a root is registered, so
/// membership in the set is the circuit's job. Same construction as the pool
/// tree: a spend that cannot open against a published set has no proof.
pub fn assoc_membership(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    others: &[u64],
    unlisted: bool,
    depth: usize,
) -> Assoc {
    let mut tree = PoolTree::with_depth(h.clone(), depth);
    for s in others {
        let mut pad = [Fp::ZERO; RATE];
        pad[0] = Fp::from_u64(*s);
        tree.insert(pad);
    }
    let idx: Vec<usize> = cms.iter().map(|cm| tree.insert(*cm)).collect();
    let root = tree.root();
    let openings: Vec<Witnessed> = (0..2)
        .map(|i| {
            let at = if unlisted { idx[1 - i] } else { idx[i] };
            Witnessed {
                leaf_index: at,
                siblings: tree.path(at).0,
            }
        })
        .collect();
    build_assoc(h, cms, [&openings[0], &openings[1]], root, unlisted)
}

/// Association membership against a root the registry published.
///
/// The planted form above builds its own set and hands back the root it
/// computed, which is fine for a gate and useless against a pool: the
/// registry holds the root a settlement is checked against, and a
/// prover-invented one is refused by `isRegisteredRoot`. This is the same
/// split the note tree already has between `pool_membership` and
/// `pool_membership_against`, and both forms go through one `build_assoc`
/// so the two cannot drift.
pub fn assoc_membership_against(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    openings: [&Witnessed; 2],
    root: [Fp; RATE],
) -> Assoc {
    for (i, o) in openings.iter().enumerate() {
        assert!(!o.siblings.is_empty(), "association opening {i} is empty");
    }
    build_assoc(h, cms, openings, root, false)
}

fn build_assoc(
    h: &Poseidon,
    cms: &[[Fp; RATE]; 2],
    openings: [&Witnessed; 2],
    root: [Fp; RATE],
    unlisted: bool,
) -> Assoc {
    let mut regions: Vec<ShieldRegion> = Vec::with_capacity(2);
    let mut traces = Vec::with_capacity(2);
    let mut leaf_col = Vec::with_capacity(2);
    for (i, cm) in cms.iter().enumerate() {
        // A note that really is in the set, but not the note being spent.
        let claimed = if unlisted { cms[1 - i] } else { *cm };
        let o = openings[i];
        let dirs: Vec<bool> = (0..o.siblings.len())
            .map(|m| (o.leaf_index >> m) & 1 == 1)
            .collect();
        let m = note_member(h, claimed, o.siblings.clone(), dirs, root);
        /*
         * The canonical leaf column, not the half the bottom direction
         * happens to select.
         *
         * This read `if dirs[0] { RATE } else { 0 }`, which names whichever
         * half of the raw initial state holds the leaf. That is a position
         * and it reaches the copy constraints, so the circuit would differ
         * for notes at even and odd association positions. It did not show
         * while the set was planted identically every time; it would have
         * appeared the moment a real registry supplied the positions, which
         * is this change. The note side has always used the pinned column
         * and this now matches it.
         */
        leaf_col.push(m.region.leaf_col());
        regions.push(ShieldRegion::Membership(m.region));
        traces.push(m.witness);
    }
    Assoc {
        regions,
        traces,
        leaf_col,
        root,
    }
}
