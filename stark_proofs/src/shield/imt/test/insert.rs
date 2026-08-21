// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::{chain, writes_are_distinct, Leaf, Low, Step};

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

fn tree() -> [Leaf; 2] {
    [leaf(0, 100, false), leaf(100, 0, true)]
}

/// Keys landing in separate gaps each take a leaf already in the tree.
#[test]
fn keys_in_separate_gaps_take_existing_low_leaves() {
    let s = chain(&[key(50), key(150)], &tree()).unwrap();
    assert!(matches!(s[0].low, Low::InTree(0)));
    assert!(matches!(s[1].low, Low::InTree(1)));
}

/// Two keys in one gap. Without sorting they would both mutate leaf zero; sorted,
/// the second takes the first as its low leaf and the run is a chain.
#[test]
fn sort_adjacent_keys_chain_instead_of_colliding() {
    let s = chain(&[key(50), key(60)], &tree()).unwrap();
    assert!(matches!(s[0].low, Low::InTree(0)));
    assert!(matches!(s[1].low, Low::InBatch(0)));
}

/// A duplicate cannot survive a strictly increasing chain, so uniqueness within
/// the batch is the shape rather than a rule laid on top of it.
#[test]
fn a_repeated_key_has_no_chain() {
    assert!(chain(&[key(50), key(50)], &tree()).is_none());
}

/// Out of order is refused rather than sorted for the caller: the circuit proves
/// the order, so a run that is not in it has nothing to prove.
#[test]
fn an_unsorted_batch_has_no_chain() {
    assert!(chain(&[key(60), key(50)], &tree()).is_none());
}

/// Every key above the maximum still chains, each taking the one before it.
#[test]
fn a_run_past_the_last_leaf_chains() {
    let s = chain(&[key(150), key(160), key(170)], &tree()).unwrap();
    assert!(matches!(s[0].low, Low::InTree(1)));
    assert!(matches!(s[1].low, Low::InBatch(0)));
    assert!(matches!(s[2].low, Low::InBatch(1)));
}

/// Below every leaf there is no low leaf, so the genesis sentinel at zero is what
/// makes the first insert possible at all.
#[test]
fn a_key_below_the_sentinel_has_no_chain() {
    let only = [leaf(10, 0, true)];
    assert!(chain(&[key(5)], &only).is_none());
}

/// Distinctness is a consequence of same-gap chaining, not a property of its own.
/// The chain gives it: the second key takes the first as its low leaf.
#[test]
fn a_chained_batch_writes_distinct_leaves() {
    let s = chain(&[key(50), key(60)], &tree()).unwrap();
    assert!(writes_are_distinct(&s));
}

/// And the refactor that severs it. Validating each key against the pre-batch
/// tree looks right, since both satisfy L.value < key < L.nextValue, and both
/// then write L.next. Separate gaps stay green, so the day it lands nothing says
/// so. This is the check that says so.
#[test]
fn two_same_gap_keys_claiming_the_pre_batch_leaf_are_refused() {
    let pre_batch = alloc::vec![
        Step {
            key: key(50),
            low: Low::InTree(0)
        },
        Step {
            key: key(60),
            low: Low::InTree(0)
        },
    ];
    assert!(
        !writes_are_distinct(&pre_batch),
        "both keys wrote one leaf, so the fold would lose one of them"
    );
}
