// NONOS Operating System (AGPL-3.0-or-later)

use super::scenario::balanced;
use crate::shield::key::Break;
use crate::witness_satisfies::satisfies;

/// A note genuinely owned and genuinely a member, retired under a position the
/// pool never authenticated. Flips the lowest bit alone, which is the one the
/// opening consumes building its initial state. Two of these are one note spent
/// twice.
#[test]
fn a_note_cannot_be_retired_under_a_foreign_index() {
    let js = balanced(Break::ForeignIndex);
    assert!(
        !satisfies(&js.wired, &js.witness),
        "a note was retired under a leaf index the pool never authenticated, \
         which is a double spend"
    );
}
