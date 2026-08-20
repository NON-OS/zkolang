// NONOS Operating System (AGPL-3.0-or-later)

use super::scenario::balanced;
use crate::shield::key::Break;
use crate::witness_satisfies::satisfies;

/// The published `noteRoot` is compared to the root the *first* note's membership
/// walked to. The second note walks its own path to its own root, and that root is
/// computed and then dropped.
///
/// So the second note here is a member of a tree holding it and nothing else. It is
/// genuinely committed, genuinely owned, its nullifier is genuinely derived. It is
/// simply not in the pool. If that satisfies, a spender mints by pairing one real
/// note with one invented one.
#[test]
fn a_second_note_cannot_walk_to_a_pool_of_its_own() {
    let js = balanced(Break::ForeignPoolRoot);
    assert!(
        !satisfies(&js.wired, &js.witness),
        "the second note proved membership of a tree nobody published, so a note \
         that was never deposited was spent"
    );
}
