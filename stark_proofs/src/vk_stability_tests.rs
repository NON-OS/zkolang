// NONOS Operating System (AGPL-3.0-or-later)
//! One deployed verifier key has to check every transfer. The key binds the
//! periodic columns, so if any of them carry witness the key moves per proof and
//! no fixed on-chain verifier can exist. Nothing caught that for a long time
//! because no test ever built two transfers and compared their AIRs.

use crate::crypto::stark::air::Air;
use crate::shield::key::Break;
use crate::shield::test::fixture::{owned, plain, secret};
use crate::shield::test::scenario::{balanced, build};

#[test]
fn two_transfers_share_one_verifier_key() {
    let a = balanced(Break::None);

    // A different spend in every respect a witness can vary: other secrets, other
    // amounts, other assets, other notes.
    let sks = [secret(7), secret(9)];
    let ins = [owned(sks[0], 3, 4000), owned(sks[1], 40, 5000)];
    let outs = [plain(50, 6000), plain(60, 2500)];
    let b = build(&ins, &outs, sks, 400, 100, Break::None, None);

    let (pa, pb) = (a.wired.periodic_columns(), b.wired.periodic_columns());
    assert_eq!(pa.len(), pb.len(), "the two transfers do not even have the same AIR shape");

    let moved: alloc::vec::Vec<usize> = pa
        .iter()
        .zip(pb.iter())
        .enumerate()
        .filter(|(_, (x, y))| x != y)
        .map(|(i, _)| i)
        .collect();
    assert!(
        moved.is_empty(),
        "{} periodic columns follow the witness, so the verifier key moves per \
         transfer and one deployed key cannot check both: {:?}",
        moved.len(),
        moved
    );
}
