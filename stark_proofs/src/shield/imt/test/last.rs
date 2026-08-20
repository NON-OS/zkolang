// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::{excludes, last_is_the_maximum, Leaf};

fn key(v: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(v);
    k
}

fn leaf(v: u64, next: u64, is_last: bool) -> Leaf {
    Leaf {
        value: key(v),
        next_index: 0,
        next_value: if is_last { [Fp::ZERO; RATE] } else { key(next) },
        is_last,
    }
}

#[test]
fn one_last_leaf_holding_the_largest_key_is_well_formed() {
    assert!(last_is_the_maximum(&[leaf(0, 10, false), leaf(10, 20, false), leaf(20, 0, true)]));
}

/// Two last leaves. Everything above the lower one looks excluded.
#[test]
fn a_second_last_leaf_rejects() {
    assert!(!last_is_the_maximum(&[leaf(0, 10, false), leaf(10, 0, true), leaf(20, 0, true)]));
}

/// One last leaf, in the middle. This is the forgery worth having: the key 30 is
/// in the set at leaf 20, and a non-membership proof against the middle leaf
/// still passes, because is_last drops the upper bound.
#[test]
fn a_last_leaf_that_is_not_the_maximum_rejects() {
    let set = [leaf(0, 10, false), leaf(10, 0, true), leaf(20, 0, false)];
    assert!(!last_is_the_maximum(&set));
    assert!(
        excludes(&set[1].value, &set[1].next_value, set[1].is_last, &key(30)),
        "the range check alone cannot see it, which is why the shape is its own rule"
    );
}

/// No last leaf at all: the chain has no top and the largest key has nothing
/// pointing past it.
#[test]
fn a_set_with_no_last_leaf_rejects() {
    assert!(!last_is_the_maximum(&[leaf(0, 10, false), leaf(10, 20, false)]));
}

/// The last leaf's own next is unused, so it is required zero rather than left to
/// the witness.
#[test]
fn a_last_leaf_pointing_somewhere_rejects() {
    let mut top = leaf(20, 0, true);
    top.next_value = key(99);
    assert!(!last_is_the_maximum(&[leaf(0, 20, false), top]));
}
