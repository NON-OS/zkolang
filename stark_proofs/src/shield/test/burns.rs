// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, plain, secret};
use super::satisfies::satisfies;
use super::scenario::build;
use crate::shield::key::Break;

/// The total is pinned at both ends, so an unbalanced batch fails either way.
#[test]
fn destroying_value_rejects() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1), plain(30, 1)];
    let js = build(&ins, &outs, sks, 200, 100, Break::None);
    assert!(!satisfies(&js.wired, &js.witness));
}
