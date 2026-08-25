// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::shield::agg::{combine, induced, lift, Carried, Effect, Verified};
use crate::shield::note::POOL_LOG_ROUNDS;

fn h() -> Poseidon {
    Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE])
}

fn d(x: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(x);
    k
}

fn genesis() -> Carried {
    Carried {
        note_root: d(1),
        next_index: 0,
        nullifier_root: d(2),
    }
}

fn spend(a: u64, b: u64) -> Effect {
    Effect {
        nullifiers: [d(a), d(b)],
        outputs: [d(a + 50), d(b + 50)],
    }
}

/// The node exposes the transition its own proof induces.
#[test]
fn a_lift_exposing_the_induced_transition_holds() {
    let (h, old, e) = (h(), genesis(), spend(10, 11));
    assert!(lift(&h, old, &e, induced(&h, old, &e)).is_some());
}

/// The forgery the seam owes. The proof retires 10 and 11; the node exposes the
/// transition that retiring 20 and 21 would make. One verified proof, a different
/// move composed, and everything above carries it as fact.
#[test]
fn a_lift_exposing_someone_elses_transition_does_not() {
    let (h, old) = (h(), genesis());
    let other = induced(&h, old, &spend(20, 21));
    assert!(
        lift(&h, old, &spend(10, 11), other).is_none(),
        "the node published a move its proof did not make"
    );
}

/// Retire the proof's nullifiers, append outputs of the node's choosing.
#[test]
fn a_lift_swapping_the_outputs_does_not() {
    let (h, old, e) = (h(), genesis(), spend(10, 11));
    let mut forged = e;
    forged.outputs = [d(999), d(998)];
    assert!(lift(&h, old, &e, induced(&h, old, &forged)).is_none());
}

/// Two notes in, two out, so the index moves by two. A node that leaves it where
/// it was has the next batch overwrite what this one appended.
#[test]
fn a_lift_that_does_not_advance_the_index_does_not() {
    let (h, old, e) = (h(), genesis(), spend(10, 11));
    let mut stuck = induced(&h, old, &e);
    stuck.next_index = old.next_index;
    assert!(lift(&h, old, &e, stuck).is_none());
}

/// Two lifted leaves compose when the second started where the first ended, and
/// the values compared are the ones the children exposed.
#[test]
fn two_lifted_children_compose() {
    let h = h();
    let a = lift(
        &h,
        genesis(),
        &spend(10, 11),
        induced(&h, genesis(), &spend(10, 11)),
    )
    .unwrap();
    let b = lift(
        &h,
        a.new,
        &spend(12, 13),
        induced(&h, a.new, &spend(12, 13)),
    )
    .unwrap();
    let n = combine(&Verified { exposed: a }, &Verified { exposed: b }).unwrap();
    assert!(n.old == genesis() && n.new == b.new);
}

/// Both children lifted honestly from the same state. Each is a valid transfer
/// with a real transition, and composing them still drops one, which is why the
/// chain equality sits above the lift rather than replacing it.
#[test]
fn two_lifted_children_from_one_state_do_not_compose() {
    let h = h();
    let s = genesis();
    let a = lift(&h, s, &spend(10, 11), induced(&h, s, &spend(10, 11))).unwrap();
    let b = lift(&h, s, &spend(12, 13), induced(&h, s, &spend(12, 13))).unwrap();
    assert!(combine(&Verified { exposed: a }, &Verified { exposed: b }).is_none());
}
