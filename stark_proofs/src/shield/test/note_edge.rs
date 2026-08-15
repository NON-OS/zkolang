// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced;
use crate::shield::key::Break;

/// Three individually valid compressions that are not chained are not a
/// commitment. Only the edge constraint separates chained from three strangers.
#[test]
fn an_unchained_compress_tree_is_not_a_commitment() {
    let js = balanced(Break::NoteEdge);
    assert!(!satisfies(&js.wired, &js.witness));
}
