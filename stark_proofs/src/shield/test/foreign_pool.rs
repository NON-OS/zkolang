// NONOS Operating System (AGPL-3.0-or-later)

use super::scenario::balanced;
use crate::shield::key::Break;
use crate::witness_satisfies::satisfies;

/// The second note walks to a tree holding it and nothing else. Committed, owned,
/// nullifier derived, simply not in the pool. Pair one real note with one invented
/// one and that is a mint.
#[test]
fn a_second_note_cannot_walk_to_a_pool_of_its_own() {
    let js = balanced(Break::ForeignPoolRoot);
    assert!(
        !satisfies(&js.wired, &js.witness),
        "the second note proved membership of a tree nobody published, so a note \
         that was never deposited was spent"
    );
}
