// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::plain;
use crate::shield::note::note_parts;

#[test]
fn every_limb_moves_the_commitment() {
    let base = note_parts(&plain(0, 1000)).cm;
    let mut n = plain(0, 1000);
    n.value += 1;
    assert_ne!(note_parts(&n).cm, base);
    let mut n = plain(0, 1000);
    n.asset_id += 1;
    assert_ne!(note_parts(&n).cm, base);
    for i in 0..4 {
        let mut n = plain(0, 1000);
        n.spend_pk[i] += 1;
        assert_ne!(note_parts(&n).cm, base);
        let mut n = plain(0, 1000);
        n.blinding[i] += 1;
        assert_ne!(note_parts(&n).cm, base);
    }
}
