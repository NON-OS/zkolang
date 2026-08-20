// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::imt::hash::hasher;
use crate::shield::imt::{root_of, Path, Tree};
use alloc::vec::Vec;

fn v(x: u64) -> [Fp; RATE] {
    let mut k = [Fp::ZERO; RATE];
    k[0] = Fp::from_u64(x);
    k
}

/// Leaves after the changes, and the honest sibling for every level of a path.
fn paths(changed: &[(usize, u64)]) -> (Vec<Path>, [Fp; RATE]) {
    let mut leaves: Vec<[Fp; RATE]> = (0..16u64).map(v).collect();
    for (i, x) in changed {
        leaves[*i] = v(*x);
    }
    let t = Tree::build(&hasher(), leaves.clone());
    let ps = changed
        .iter()
        .map(|(i, x)| {
            let mut idx = *i;
            let siblings = (0..t.depth)
                .map(|d| {
                    let s = t.level[d][idx ^ 1];
                    idx >>= 1;
                    s
                })
                .collect();
            Path { index: *i, leaf: v(*x), siblings }
        })
        .collect();
    (ps, t.root())
}

/// Two co-ancestral leaves, each carrying the honest sibling for the node they
/// share. This is the accept to review on: a fold that only handles disjoint
/// paths refuses it.
#[test]
fn co_ancestral_paths_agreeing_on_the_shared_node_fold() {
    let (ps, root) = paths(&[(4, 40), (5, 50)]);
    assert_eq!(root_of(&hasher(), &ps), Some(root));
}

/// Four under one small subtree, sharing ancestors at three levels.
#[test]
fn a_dense_cluster_of_paths_folds() {
    let (ps, root) = paths(&[(4, 40), (5, 50), (6, 60), (7, 70)]);
    assert_eq!(root_of(&hasher(), &ps), Some(root));
}

#[test]
fn scattered_paths_fold() {
    let (ps, root) = paths(&[(0, 90), (3, 91), (9, 92), (14, 93)]);
    assert_eq!(root_of(&hasher(), &ps), Some(root));
}

/// The forgery the whole thing owes. Both leaves are changed, both paths are
/// internally consistent, and they disagree about the ancestor they share, so
/// each reaches a root of its own and neither is the tree's.
#[test]
fn co_ancestral_paths_disagreeing_on_the_shared_node_do_not() {
    let (mut ps, _) = paths(&[(4, 40), (6, 60)]);
    // Leaves 4 and 6 share the node at level two. Move what the second path
    // claims for its level-one sibling and that shared node parts company.
    ps[1].siblings[1] = v(0xDEAD);
    assert!(
        root_of(&hasher(), &ps).is_none(),
        "two paths claimed different values for one node and both reached a root"
    );
}

/// A sibling that is wrong but shared with nobody still fails, through the roots
/// disagreeing rather than through the node check.
#[test]
fn a_lone_path_with_a_wrong_sibling_does_not() {
    let (mut ps, _) = paths(&[(4, 40), (11, 80)]);
    ps[1].siblings[0] = v(0xBEEF);
    assert!(root_of(&hasher(), &ps).is_none());
}

/// One path is always consistent with itself.
#[test]
fn a_single_path_folds() {
    let (ps, root) = paths(&[(7, 70)]);
    assert_eq!(root_of(&hasher(), &ps), Some(root));
}
