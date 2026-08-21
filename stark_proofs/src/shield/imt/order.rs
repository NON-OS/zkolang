// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use core::cmp::Ordering;

/// The order the contract compares in: the four limbs as one 256 bit integer,
/// little endian. Every limb of a nullifier is below p, so the same comparison
/// runs as `uint256 <` on chain and as a limb chain in circuit without either
/// side translating.
pub(crate) fn cmp(a: &[Fp; RATE], b: &[Fp; RATE]) -> Ordering {
    for i in (0..RATE).rev() {
        match a[i].value().cmp(&b[i].value()) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

/// Non-membership: the low leaf sits strictly below the key, and its neighbour
/// strictly above, or it is the last leaf and nothing is above it.
///
/// Both bounds strict. `v == low.value` is the key already in the set, which is a
/// double spend; `v == low.next_value` is the next leaf's key, which is the same.
/// They fail through different comparisons, so each carries its own forgery.
pub(crate) fn excludes(low: &[Fp; RATE], next: &[Fp; RATE], is_last: bool, v: &[Fp; RATE]) -> bool {
    cmp(low, v) == Ordering::Less && (is_last || cmp(v, next) == Ordering::Less)
}
