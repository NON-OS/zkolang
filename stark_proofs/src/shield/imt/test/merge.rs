// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::{stitch, Range};

fn key(v: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(v);
    k
}

fn range(low: u64, sets: &[(u64, u64)]) -> Range {
    Range {
        low: key(low),
        sets: sets.iter().map(|(f, t)| (key(*f), key(*t))).collect(),
    }
}

/// Different gaps: A follows leaf 0, B follows leaf 100, neither touches the
/// other's leaf and B starts from a leaf A left alone.
#[test]
fn ranges_in_separate_gaps_merge() {
    let a = range(0, &[(0, 10), (10, 100)]);
    let b = range(100, &[(100, 150), (150, 200)]);
    assert!(stitch(&a, &b).is_some());
}

/// The seam. Both ranges land between leaf 0 and leaf 100, so both were computed
/// believing they set leaf 0's pointer. Merged as written, B's write to leaf 0
/// lands on top of A's and A's whole range leaves the chain, while the product
/// still closes.
#[test]
fn ranges_sharing_a_gap_do_not_merge_as_written() {
    let a = range(0, &[(0, 10), (10, 100)]);
    let b = range(0, &[(0, 50), (50, 100)]);
    assert!(stitch(&a, &b).is_none(), "two subtrees wrote the same leaf and one range vanished");
}

/// Stitched: B follows A's last key rather than the leaf they both started from,
/// which is A.new == B.old carried across the seam.
#[test]
fn a_stitched_seam_merges() {
    let a = range(0, &[(0, 10), (10, 20)]);
    let b = range(20, &[(20, 50), (50, 100)]);
    assert!(stitch(&a, &b).is_some());
}

/// Out of order across the seam: B's first key is below A's last, so the chain
/// would not be increasing through the join.
#[test]
fn a_seam_that_runs_backwards_does_not_merge() {
    let a = range(0, &[(0, 50), (50, 100)]);
    let b = range(50, &[(50, 10), (10, 100)]);
    assert!(stitch(&b, &a).is_none());
}

/// B starting from a leaf A pointed at, without following A's last key, leaves
/// A's tail and B's head claiming the same successor.
#[test]
fn a_seam_that_reuses_a_pointer_target_does_not_merge() {
    let a = range(0, &[(0, 10), (10, 100)]);
    let b = range(100, &[(100, 10), (10, 200)]);
    assert!(stitch(&a, &b).is_none());
}

/// An empty side has no seam to check and nothing to stitch to.
#[test]
fn an_empty_range_does_not_merge() {
    let a = range(0, &[(0, 10)]);
    assert!(stitch(&a, &range(10, &[])).is_none());
}
