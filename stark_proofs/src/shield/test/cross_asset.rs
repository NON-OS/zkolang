// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{owned, secret};
use super::scenario::build;
use crate::shield::key::Break;
use crate::shield::note::Note;
use crate::witness_satisfies::satisfies;

/// Conservation sums values. It does not sum them per asset, and the public asset
/// word is pinned to the first input only, so the other three notes carry whatever
/// asset they like into the same total.
///
/// Here the second input is a different asset. Value conserves, every note is
/// genuinely committed, every spend is genuinely owned, and the outputs are minted
/// in the first asset. If it satisfies, a junk note of one asset has been converted
/// into value in another.
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
