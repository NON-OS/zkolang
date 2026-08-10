// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{plain, secret};
use super::satisfies::satisfies;
use super::scenario::build;
use crate::shield::key::Break;

/// Spend a note whose committed key this secret never derived. The key
/// hierarchy is internally honest; only the tie to the commitment fails.
#[test]
fn a_secret_cannot_spend_a_note_it_does_not_key() {
    let sks = [secret(1), secret(2)];
    let ins = [plain(0, 1000), plain(10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    let js = build(&ins, &outs, sks, 200, 100, Break::None, None);
    assert!(!satisfies(&js.wired, &js.witness));
}
