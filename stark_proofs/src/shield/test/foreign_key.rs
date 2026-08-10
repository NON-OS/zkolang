// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced;
use crate::shield::key::Break;

/// Double spend by alternate key. Every compression stays honest, so only the
/// shared secret binding can reject.
#[test]
fn a_second_key_cannot_retire_the_note() {
    let js = balanced(Break::ForeignKey);
    assert!(!satisfies(&js.wired, &js.witness));
}
