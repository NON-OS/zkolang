// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, plain, secret};
use crate::shield::join::{join_split_placed, Placed, Places, Settle, Spend};
use crate::shield::member::PoolTree;
use crate::shield::note::{note_parts, Note, POOL_LOG_ROUNDS};
use crate::crypto::stark::air::{Poseidon, RATE};
use crate::crypto::stark::field::Fp;
use crate::witness_satisfies::satisfies;

/// A spend against a pool the caller already holds, which is how a wallet spends:
/// the contract owns the tree and publishes the root, the wallet reads the paths.
/// The tree here stands in for the contract's, with other people's notes in it.
#[test]
fn a_spend_proves_against_a_pool_it_did_not_build() {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];

    // Deposits that are not ours, so the notes do not land at zero and one.
    let mut tree = PoolTree::with_depth(h.clone(), super::depth::MINIMAL);
    for seed in [700u64, 701, 702] {
        tree.insert(note_parts(&plain(seed, 1)).cm);
    }
    // Both in, then both paths. A path authenticates the root it was read at, so
    // reading one before the next insert authenticates a tree that is already old.
    let at = {
        let idx = [tree.insert(note_parts(&ins[0]).cm), tree.insert(note_parts(&ins[1]).cm)];
        let mut place = |i: usize| {
            let (siblings, directions) = tree.path(i);
            Placed { siblings, directions, leaf_index: i }
        };
        Places { note: [place(idx[0]), place(idx[1])], root: tree.root() }
    };

    let js = join_split_placed(
        [Spend { note: &ins[0], sk: sks[0] }, Spend { note: &ins[1], sk: sks[1] }],
        [&outs[0], &outs[1]],
        200,
        100,
        Settle { clearing_price: 1_000_000, recipient: 0xBEEF },
        &at,
    );
    assert!(satisfies(&js.wired, &js.witness), "a spend against the contract's pool failed");
}
