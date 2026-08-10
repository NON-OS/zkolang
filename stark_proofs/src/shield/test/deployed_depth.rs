// NONOS Operating System (AGPL-3.0-or-later)

use super::depth::DEPLOYED;
use super::fixture::{owned, plain, secret};
use super::satisfies::satisfies;
use crate::shield::join::{join_split_at, Settle, Spend};
use crate::shield::key::Break;

/// The binding suite runs on a minimal instance so it stays a change gate. This
/// is the same statement at the deployed depth, kept as the release gate: if the
/// two ever disagree the minimal instance has stopped standing in for the real
/// one.
#[test]
#[ignore]
fn the_bindings_hold_at_the_deployed_depth() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), owned(sks[1], 10, 2000)];
    let outs = [plain(20, 1500), plain(30, 1200)];
    let st = Settle { clearing_price: 1_000_000, recipient: 0xBEEF };
    let mk = |brk| {
        join_split_at(
            DEPLOYED,
            [Spend { note: &ins[0], sk: sks[0] }, Spend { note: &ins[1], sk: sks[1] }],
            [&outs[0], &outs[1]],
            200,
            100,
            brk,
            st,
            None,
        )
    };
    let honest = mk(Break::None);
    assert!(satisfies(&honest.wired, &honest.witness), "honest must hold at depth 32");
    for brk in [Break::ForeignKey, Break::ForeignNote, Break::NoteEdge, Break::Unlisted] {
        let js = mk(brk);
        assert!(!satisfies(&js.wired, &js.witness), "a forgery survived at depth 32");
    }
}
