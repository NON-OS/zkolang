// NONOS Operating System (AGPL-3.0-or-later)

use super::tree::TREE_DEPTH;
use crate::crypto::stark::air::{Air, MultiMembership, Opening, Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::note::POOL_LOG_ROUNDS;
use alloc::vec::Vec;

pub(crate) struct NoteMember {
    pub region: MultiMembership,
    pub witness: Vec<Fp>,
    pub proven_root: [Fp; RATE],
}

/// The constraints do not enforce membership: a tampered path is honest
/// arithmetic that walks elsewhere. Membership is the walked root equalling the
/// published one, so the root is read back out of the trace rather than trusted.
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
    let w = region.trace_width();
    let row = TREE_DEPTH * (1usize << POOL_LOG_ROUNDS);
    let mut proven_root = [Fp::ZERO; RATE];
    proven_root.copy_from_slice(&witness[row * w..row * w + RATE]);
    NoteMember { region, witness, proven_root }
}
