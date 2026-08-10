// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, plain, secret};
use super::satisfies::satisfies;
use super::scenario::build;
use crate::shield::key::Break;

#[test]
fn creating_value_from_nothing_rejects() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 99_999)];
    let js = build(&ins, &outs, sks, 200, 100, Break::None, None);
    assert!(!satisfies(&js.wired, &js.witness));
}
