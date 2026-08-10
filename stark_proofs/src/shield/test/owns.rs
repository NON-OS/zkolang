// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced;
use crate::shield::key::Break;

/// Theft from seeing a commitment: retire a note the key does not open.
#[test]
#[ignore]
fn a_key_cannot_retire_a_note_it_does_not_open() {
    let js = balanced(Break::ForeignNote);
    assert!(!satisfies(&js.wired, &js.witness));
}
