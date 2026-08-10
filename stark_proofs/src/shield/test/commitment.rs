// NONOS Operating System (AGPL-3.0-or-later)

use super::fixture::{hasher, plain};
use crate::shield::note::note_parts;

#[test]
fn the_circuit_commitment_is_the_pool_commitment() {
    let n = plain(0, 1234);
    assert_eq!(note_parts(&n).cm, hasher().commit_note(&n.limbs()));
}
