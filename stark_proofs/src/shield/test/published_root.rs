// NONOS Operating System (AGPL-3.0-or-later)
//! Spending against a root the pool published, rather than one the builder
//! computed on the way past.
//!
//! Every join split before this planted its own tree, inserted the two notes it
//! was about to spend, and proved against the root that fell out. The statement
//! was true and useless: it says the notes belong to a tree containing exactly
//! those notes, under a root nobody else has ever seen. Here the tree holds
//! other people's notes as well, the root is taken before the spend is built,
//! and the builder is handed openings into it.

use super::depth::MINIMAL;
use super::fixture::{hasher, owned, plain, secret};
use super::satisfies::satisfies;
use crate::crypto::stark::air::RATE;
use crate::crypto::stark::field::Fp;
use crate::shield::join::address_from_u64;
use crate::shield::join::{join_split_with_paths, Settle, Spend, Witnessed};
use crate::shield::key::Break;
use crate::shield::member::PoolTree;
use crate::shield::note::{note_parts, Note};
use alloc::vec::Vec;

/// A pool with strangers in it, the two spent notes somewhere inside, and the
/// root read off before anything is proven.
fn pool_with(ins: &[Note; 2]) -> (PoolTree, [usize; 2], [Fp; RATE]) {
    let h = hasher();
    let mut t = PoolTree::with_depth(h, MINIMAL);
    // Strangers first, so neither spent note lands at position zero and the
    // direction bits are not all the same value.
    for s in 0..3u64 {
        t.insert(note_parts(&plain(40 + s, 500 + s)).cm);
    }
    let a = t.insert(note_parts(&ins[0]).cm);
    t.insert(note_parts(&plain(60, 700)).cm);
    let b = t.insert(note_parts(&ins[1]).cm);
    for s in 0..2u64 {
        t.insert(note_parts(&plain(70 + s, 800 + s)).cm);
    }
    let root = t.root();
    (t, [a, b], root)
}

fn opening(t: &PoolTree, i: usize) -> Witnessed {
    Witnessed {
        leaf_index: i,
        siblings: t.path(i).0,
    }
}

#[test]
fn a_spend_proves_against_a_root_the_pool_published() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    let (t, at, root) = pool_with(&ins);
    let (o0, o1) = (opening(&t, at[0]), opening(&t, at[1]));

    let js = join_split_with_paths(
        MINIMAL,
        [
            Spend {
                note: &ins[0],
                sk: sks[0],
            },
            Spend {
                note: &ins[1],
                sk: sks[1],
            },
        ],
        [&o0, &o1],
        root,
        [&outs[0], &outs[1]],
        200,
        100,
        Break::None,
        Settle {
            clearing_price: 1_000_000,
            recipient: address_from_u64(0xBEEF),
        },
        None,
    );
    assert!(
        satisfies(&js.wired, &js.witness),
        "an honest spend against a published root failed"
    );
}

#[test]
fn the_published_root_is_not_one_the_builder_could_have_invented() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let (t, at, root) = pool_with(&ins);
    /*
     * The point of the whole change, stated as an assertion: a tree holding
     * only the two spent notes reaches a different root, so a proof that
     * verifies against the published one cannot have been built by planting a
     * tree around its own inputs. Without this the new path could silently
     * degrade back into the old one and every other test here would still pass.
     */
    let h = hasher();
    let mut alone = PoolTree::with_depth(h, MINIMAL);
    alone.insert(note_parts(&ins[0]).cm);
    alone.insert(note_parts(&ins[1]).cm);
    assert_ne!(
        alone.root(),
        root,
        "the published root must not be the planted one"
    );
    assert!(
        at[0] != 0 || at[1] != 1,
        "the spent notes must not sit where a planted tree puts them"
    );
    let _ = t;
}

#[test]
fn a_path_from_the_wrong_position_does_not_reach_the_root() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    let (t, at, root) = pool_with(&ins);
    /*
     * The siblings of the real position under a neighbouring position. The walk
     * is honest arithmetic either way, which is exactly why the constraints
     * cannot catch it: membership is the walked root equalling the published
     * one, so this must fail as a root mismatch rather than as a bad trace.
     */
    let wrong = Witnessed {
        leaf_index: at[0] ^ 1,
        siblings: t.path(at[0]).0,
    };
    let o1 = opening(&t, at[1]);
    let js = join_split_with_paths(
        MINIMAL,
        [
            Spend {
                note: &ins[0],
                sk: sks[0],
            },
            Spend {
                note: &ins[1],
                sk: sks[1],
            },
        ],
        [&wrong, &o1],
        root,
        [&outs[0], &outs[1]],
        200,
        100,
        Break::None,
        Settle {
            clearing_price: 1_000_000,
            recipient: address_from_u64(0xBEEF),
        },
        None,
    );
    assert!(
        !satisfies(&js.wired, &js.witness),
        "a note opened at the wrong position was accepted against the published root"
    );
}

#[test]
fn every_opening_walks_the_full_depth() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let (t, at, _root) = pool_with(&ins);
    let o: Vec<[Fp; RATE]> = t.path(at[0]).0;
    assert_eq!(
        o.len(),
        MINIMAL,
        "an opening must carry one sibling per level"
    );
}
