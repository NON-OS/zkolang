// NONOS Operating System (AGPL-3.0-or-later)

use super::scenario::balanced;
use crate::shield::key::Break;
use crate::witness_satisfies::satisfies;

/// A nullifier is `compress(compress(nk, cm), leaf_index)`, and `leaf_index` is
/// the note's position in the pool. The position the pool authenticates is its
/// path directions; the position the nullifier hashes is a scalar the witness
/// carries. Nothing ties them unless a binding does.
///
/// This spends a note that is genuinely owned and genuinely a member, retired
/// under a position one away from the one membership proved. If it satisfies,
/// the same note retires again under another position, and a note that retires
/// twice is a note spent twice.
#[test]
fn a_note_cannot_be_retired_under_a_foreign_index() {
    let js = balanced(Break::ForeignIndex);
    assert!(
        !satisfies(&js.wired, &js.witness),
        "a note was retired under a leaf index the pool never authenticated, \
         which is a double spend"
    );
}
