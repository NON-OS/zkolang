// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::excludes;

fn key(lo: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(lo);
    k
}

#[test]
fn a_key_between_its_neighbours_is_excluded() {
    assert!(excludes(&key(10), &key(20), false, &key(15)));
}

/// The key is the low leaf. It is in the set, so it has been spent.
#[test]
fn a_key_equal_to_the_low_leaf_is_not() {
    assert!(!excludes(&key(10), &key(20), false, &key(10)));
}

/// The key is the next leaf. Also in the set, and it fails through the other
/// comparison, which is why it is its own case.
#[test]
fn a_key_equal_to_the_next_leaf_is_not() {
    assert!(!excludes(&key(10), &key(20), false, &key(20)));
}

/// Past the last leaf nothing is above, so only the low bound applies.
#[test]
fn a_key_above_the_last_leaf_is_excluded() {
    assert!(excludes(&key(10), &key(0), true, &key(9_999)));
}

/// A high limb outranks every low one: the order is the contract's uint256, not
/// limb zero.
#[test]
fn the_order_reads_the_high_limb_first() {
    let mut big = [Fp::ZERO; RATE];
    big[3] = Fp::from_u64(1);
    assert!(excludes(&key(u64::MAX - 1), &big, false, &key(u64::MAX)));
}
