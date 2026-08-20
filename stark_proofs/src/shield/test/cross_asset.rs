// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, secret};
use super::scenario::build;
use crate::shield::key::Break;
use crate::shield::note::Note;
use crate::witness_satisfies::satisfies;

/// The second input is a different asset. Value conserves, every note is committed,
/// every spend is owned, and the outputs mint in the first asset. That converts a
/// junk asset into a real one.
#[test]
fn value_cannot_cross_between_assets() {
    let sks = [secret(1), secret(2)];
    let ins = [owned(sks[0], 0, 1000), {
        let mut n = owned(sks[1], 10, 2000);
        n.asset_id = 7;
        n
    }];
    let outs = [
        Note { value: 1500, asset_id: 0, spend_pk: [21, 22, 23, 24], blinding: [25, 26, 27, 28] },
        Note { value: 1200, asset_id: 0, spend_pk: [31, 32, 33, 34], blinding: [35, 36, 37, 38] },
    ];
    let js = build(&ins, &outs, sks, 200, 100, Break::None, None);
    assert!(
        !satisfies(&js.wired, &js.witness),
        "value crossed from one asset into another, so a note of a worthless asset \
         mints value in a real one"
    );
}
