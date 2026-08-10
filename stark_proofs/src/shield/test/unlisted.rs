// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced;
use crate::shield::key::Break;

/// Prove the association set holds a note that is genuinely in it, but is not
/// the note being spent. The walk reaches the published root either way.
#[test]
#[ignore]
fn a_listed_note_cannot_stand_in_for_the_spent_one() {
    let js = balanced(Break::Unlisted);
    assert!(!satisfies(&js.wired, &js.witness));
}
