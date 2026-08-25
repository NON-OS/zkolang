// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::agg::{realised, Effect};
use crate::shield::imt::Set;
use crate::shield::note::POOL_LOG_ROUNDS;

fn h() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn d(x: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(x);
    k
}

fn spend(a: u64, b: u64) -> Effect {
    Effect {
        nullifiers: [d(a), d(b)],
        outputs: [d(a + 500), d(b + 500)],
    }
}

/// A batch the tree can realise: every nullifier is new, so every one finds a gap.
#[test]
fn a_batch_the_tree_can_make_has_roots() {
    let (h, set) = (h(), Set::genesis(16));
    assert!(realised(&h, &set, d(1), 0, &[spend(10, 11), spend(12, 13)]).is_some());
}

/// The join forgery. Two transfers retiring one note compose as a chain without
/// complaint, and the tree cannot insert the second: there is no gap left. The
/// state carry alone would have carried it.
#[test]
fn a_batch_retiring_one_note_twice_has_none() {
    let (h, set) = (h(), Set::genesis(16));
    assert!(
        realised(&h, &set, d(1), 0, &[spend(10, 11), spend(10, 12)]).is_none(),
        "the chain composed a move the tree cannot make"
    );
}

/// A nullifier already in the set has no gap either, which is the double spend
/// across batches rather than within one.
#[test]
fn a_batch_retiring_a_spent_note_has_none() {
    let (h, set) = (h(), Set::genesis(16));
    let after = set.insert(&[d(10), d(11)]).unwrap();
    assert!(realised(&h, &after, d(1), 4, &[spend(10, 20)]).is_none());
}

/// The order the transfers arrive in cannot reach the roots, so the sequencer
/// batches them any way it likes.
#[test]
fn the_order_the_transfers_arrive_in_cannot_reach_the_roots() {
    let (h, set) = (h(), Set::genesis(16));
    let a = realised(&h, &set, d(1), 0, &[spend(10, 11), spend(12, 13)]).unwrap();
    let b = realised(&h, &set, d(1), 0, &[spend(12, 13), spend(10, 11)]).unwrap();
    assert_eq!(a.1.nullifier_root, b.1.nullifier_root);
}

/// Every retired note moves the nullifier root, so one dropped from the update is
/// visible rather than silent.
#[test]
fn dropping_a_retired_note_moves_the_root() {
    let (h, set) = (h(), Set::genesis(16));
    let both = realised(&h, &set, d(1), 0, &[spend(10, 11), spend(12, 13)]).unwrap();
    let one = realised(&h, &set, d(1), 0, &[spend(10, 11)]).unwrap();
    assert_ne!(both.1.nullifier_root, one.1.nullifier_root);
}

/// More keys than the tree holds is refused rather than wrapped.
#[test]
fn a_batch_past_the_trees_capacity_has_none() {
    let (h, set) = (h(), Set::genesis(4));
    let many: alloc::vec::Vec<Effect> = (0..4).map(|i| spend(10 + i * 2, 11 + i * 2)).collect();
    assert!(realised(&h, &set, d(1), 0, &many).is_none());
}
