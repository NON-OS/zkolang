// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced;
use crate::shield::key::Break;

#[test]
fn a_conserving_owned_join_split_satisfies() {
    let js = balanced(Break::None);
    assert!(satisfies(&js.wired, &js.witness));
}
