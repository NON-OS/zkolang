// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::{stitch, Leaf, Range, State};

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

/// The chain after inserting each of `keys` into the gap they fall in.
fn insert(mut s: State, keys: &[u64]) -> State {
    for k in keys {
        let at = s
            .iter()
            .position(|l| l.value[0].value() < *k && (l.is_last || l.next_value[0].value() > *k))
            .expect("no gap");
        let (next, last) = (s[at].next_value[0].value(), s[at].is_last);
        s[at].next_value = key(*k);
        s[at].is_last = false;
        s.push(leaf(*k, next, last));
        s.sort_by_key(|l| l.value[0].value());
    }
    s
}

fn genesis() -> State {
    alloc::vec![leaf(0, 0, true)]
}

/// B started from the chain A left, so the two compose. Nothing here names a
/// topology: the equality is the whole condition.
#[test]
fn a_range_starting_where_the_last_one_ended_merges() {
    let s0 = genesis();
    let s1 = insert(s0.clone(), &[10, 20]);
    let s2 = insert(s1.clone(), &[30, 40]);
    let a = Range {
        old: s0,
        new: s1.clone(),
    };
    let b = Range { old: s1, new: s2 };
    assert!(stitch(&a, &b).is_some());
}

/// Keys that fall between A's, not after them. Never drawn as a case, still
/// merges, because it satisfies the equality.
#[test]
fn a_range_interleaving_with_the_last_one_merges() {
    let s0 = genesis();
    let s1 = insert(s0.clone(), &[10, 40]);
    let s2 = insert(s1.clone(), &[20, 30]);
    let a = Range {
        old: s0,
        new: s1.clone(),
    };
    let b = Range { old: s1, new: s2 };
    assert!(stitch(&a, &b).is_some());
}

/// Both started from the pre-batch chain, which is the double update: each
/// believes it owns the low leaf's pointer and the second overwrites the first.
#[test]
fn two_ranges_both_starting_from_the_pre_batch_chain_do_not_merge() {
    let s0 = genesis();
    let a = Range {
        old: s0.clone(),
        new: insert(s0.clone(), &[10, 20]),
    };
    let b = Range {
        old: s0.clone(),
        new: insert(s0, &[30, 40]),
    };
    assert!(stitch(&a, &b).is_none(), "one range's writes would vanish");
}

/// B started from a chain that is A's with one pointer moved: close enough to
/// pass a shape check, not equal.
#[test]
fn a_range_starting_from_a_doctored_chain_does_not_merge() {
    let s0 = genesis();
    let s1 = insert(s0.clone(), &[10, 20]);
    let mut doctored = s1.clone();
    doctored[1].next_value = key(99);
    let a = Range { old: s0, new: s1 };
    let b = Range {
        old: doctored.clone(),
        new: insert(doctored, &[30]),
    };
    assert!(stitch(&a, &b).is_none());
}

/// Composition is associative over the equality, so a tree of merges is the same
/// chain as a run of them. Without that, depth would change the answer.
#[test]
fn merging_is_associative() {
    let s0 = genesis();
    let s1 = insert(s0.clone(), &[10]);
    let s2 = insert(s1.clone(), &[20]);
    let s3 = insert(s2.clone(), &[30]);
    let (a, b, c) = (
        Range {
            old: s0.clone(),
            new: s1.clone(),
        },
        Range {
            old: s1,
            new: s2.clone(),
        },
        Range {
            old: s2,
            new: s3.clone(),
        },
    );
    let left = stitch(&stitch(&a, &b).unwrap(), &c).unwrap();
    let right = stitch(&a, &stitch(&b, &c).unwrap()).unwrap();
    assert!(crate::shield::imt::same(&left.new, &right.new));
    assert!(crate::shield::imt::same(&left.new, &s3));
}
