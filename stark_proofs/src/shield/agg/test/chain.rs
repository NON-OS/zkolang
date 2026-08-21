// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::agg::{chain, leaf, Carried};

fn d(x: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(x);
    k
}

/// The chain after `n` transfers: each moves both roots and advances the index by
/// its two outputs.
fn at(n: u64) -> Carried {
    Carried {
        note_root: d(100 + n),
        next_index: n * 2,
        nullifier_root: d(200 + n),
    }
}

#[test]
fn a_child_starting_where_the_last_ended_composes() {
    let a = leaf(at(0), at(1));
    let b = leaf(at(1), at(2));
    assert!(chain(&a, &b).is_some());
}

/// The forgery this whole argument rests on. Both children start from s0, the
/// parent attests s0 to the second's end, and the first child's transfers leave
/// the batch while every proof still verifies.
#[test]
fn two_children_both_starting_from_the_same_state_do_not() {
    let a = leaf(at(0), at(1));
    let b = leaf(at(0), at(2));
    assert!(
        chain(&a, &b).is_none(),
        "a subtree's transfers would vanish"
    );
}

/// The note tree moved but the nullifier set did not, so half the transition is
/// carried and half is dropped. One equality over the whole state catches it; a
/// check per field catches it only if someone wrote that field's check.
#[test]
fn a_child_agreeing_on_only_part_of_the_state_does_not() {
    let a = leaf(at(0), at(1));
    let mut half = at(1);
    half.nullifier_root = d(999);
    assert!(chain(&a, &leaf(half, at(2))).is_none());
}

/// The index is part of the state, so a child that starts at the right roots but
/// the wrong place for the next leaf does not compose either.
#[test]
fn a_child_starting_at_the_wrong_index_does_not() {
    let a = leaf(at(0), at(1));
    let mut moved = at(1);
    moved.next_index += 2;
    assert!(chain(&a, &leaf(moved, at(2))).is_none());
}

/// A tree of compositions is the same claim as a run of them, so the sequencer
/// builds it whichever parallel way it likes.
#[test]
fn composing_is_associative() {
    let (a, b, c) = (leaf(at(0), at(1)), leaf(at(1), at(2)), leaf(at(2), at(3)));
    let left = chain(&chain(&a, &b).unwrap(), &c).unwrap();
    let right = chain(&a, &chain(&b, &c).unwrap()).unwrap();
    assert!(left.old == right.old && left.new == right.new);
    assert!(left.old == at(0) && left.new == at(3));
}

/// The root exposes the pair the contract binds: where the batch started and
/// where it ended.
#[test]
fn the_root_exposes_the_pair_the_contract_binds() {
    let n = chain(
        &chain(&leaf(at(0), at(1)), &leaf(at(1), at(2))).unwrap(),
        &leaf(at(2), at(3)),
    )
    .unwrap();
    assert!(
        n.old == at(0),
        "settleAggregate requires this equals what it holds"
    );
    assert!(n.new == at(3));
}
