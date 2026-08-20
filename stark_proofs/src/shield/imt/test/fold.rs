// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::hash::hasher;
use crate::shield::imt::{refold, Tree};
use alloc::vec::Vec;

fn v(x: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(x);
    k
}

fn tree16() -> Tree {
    Tree::build(&hasher(), (0..16u64).map(v).collect())
}

/// The reference: rebuild the whole tree with the changes applied.
fn rebuilt(base: &[u64], changed: &[(usize, u64)]) -> [Fp; RATE] {
    let mut leaves: Vec<[Fp; RATE]> = base.iter().copied().map(v).collect();
    for (i, x) in changed {
        leaves[*i] = v(*x);
    }
    Tree::build(&hasher(), leaves).root()
}

fn base() -> Vec<u64> {
    (0..16u64).collect()
}

fn apply(changed: &[(usize, u64)]) -> [Fp; RATE] {
    let c: Vec<(usize, [Fp; RATE])> = changed.iter().map(|(i, x)| (*i, v(*x))).collect();
    refold(&hasher(), &tree16(), &c)
}

/// Two siblings under one parent. The minimal case, and the one a pairwise fold
/// also gets right, which is why it is not the case to review on.
#[test]
fn two_siblings_both_reach_the_root() {
    let c = [(4usize, 40u64), (5, 50)];
    assert_eq!(apply(&c), rebuilt(&base(), &c));
}

/// Four under one small subtree. A fold that combines changed pairs handles the
/// siblings above and fails closed here, which is the density trap.
#[test]
fn a_dense_cluster_all_reaches_the_root() {
    let c = [(4usize, 40u64), (5, 50), (6, 60), (7, 70)];
    assert_eq!(apply(&c), rebuilt(&base(), &c));
}

/// Scattered, which is what the pointer updates actually look like.
#[test]
fn scattered_changes_all_reach_the_root() {
    let c = [(0usize, 90u64), (3, 91), (9, 92), (14, 93)];
    assert_eq!(apply(&c), rebuilt(&base(), &c));
}

/// Dense and scattered together: a cluster plus strays, no two of which share a
/// parent. Neither shape is special to the fold.
#[test]
fn a_cluster_beside_strays_reaches_the_root() {
    let c = [(4usize, 40u64), (5, 50), (6, 60), (11, 80), (15, 85)];
    assert_eq!(apply(&c), rebuilt(&base(), &c));
}

/// The same changes in any order give the same root. Without this the shape of
/// the aggregation tree leaks into the state, and two valid orders disagree.
#[test]
fn the_order_the_changes_arrive_in_cannot_reach_the_root() {
    let forward = [(4usize, 40u64), (5, 50), (6, 60), (11, 80)];
    let shuffled = [(11usize, 80u64), (6, 60), (4, 40), (5, 50)];
    assert_eq!(apply(&forward), apply(&shuffled));
}

/// Every changed leaf moves the root, so a write that does not reach it is
/// visible rather than silent.
#[test]
fn dropping_any_one_change_moves_the_root() {
    let all = [(4usize, 40u64), (5, 50), (6, 60), (11, 80)];
    let full = apply(&all);
    for skip in 0..all.len() {
        let fewer: Vec<(usize, u64)> =
            all.iter().enumerate().filter(|(i, _)| *i != skip).map(|(_, c)| *c).collect();
        assert_ne!(apply(&fewer), full, "a change did not reach the root");
    }
}

/// Changing nothing changes nothing.
#[test]
fn an_empty_change_set_leaves_the_root() {
    assert_eq!(apply(&[]), tree16().root());
}
