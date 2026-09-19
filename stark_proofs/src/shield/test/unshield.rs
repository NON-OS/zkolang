// NONOS Operating System (AGPL-3.0-or-later)
//! The shape of a full unshield: two real notes in, nothing kept back.
//!
//! This is the first spend the pool will ever settle, so its shape is worth
//! a gate of its own before any box time goes into proving it. Two questions
//! it answers, both asked by the contracts lane while the calldata was being
//! built:
//!
//! Whether a spend with no change is a shape the circuit has at all, since
//! the join split takes two inputs and two outputs and there is no leg that
//! can be left out. It is: the outputs are notes worth zero rather than
//! absent notes.
//!
//! And whether the output commitments may be empty. They may not. The
//! circuit commits every output note through the same Poseidon the pool
//! uses, so a zero valued output still has a real commitment over its spend
//! key and blinding, and the pool will insert it as a leaf like any other.
//! An intent that sent zero bytes there would be describing a different
//! statement from the one it proved.

use super::depth::MINIMAL;
use super::fixture::{owned, plain, secret};
use super::satisfies::satisfies;
use crate::shield::join::{join_split, Settle, Spend};
use crate::shield::key::Break;

/// Two notes spent in full to a public recipient: outputs worth zero, fee
/// zero, and the public amount carrying the whole of both inputs.
#[test]
fn a_full_unshield_of_two_notes_satisfies() {
    let sks = [secret(41), secret(42)];
    let ins = [owned(sks[0], 100, 1_995_000), owned(sks[1], 200, 1_995_000)];

    /*
     * The outputs are zero valued notes, not absent ones. They still carry a
     * spend key and a blinding, so they still commit to something, and the
     * pool will store those commitments as leaves. Nobody can spend them and
     * nobody needs to; what matters is that the circuit and the calldata
     * agree that they exist.
     */
    let outs = [plain(300, 0), plain(400, 0)];

    let public_amount = 1_995_000 + 1_995_000;
    let js = crate::shield::join::join_split_at(
        MINIMAL,
        [
            Spend {
                note: &ins[0],
                sk: sks[0],
            },
            Spend {
                note: &ins[1],
                sk: sks[1],
            },
        ],
        [&outs[0], &outs[1]],
        public_amount,
        0,
        Break::None,
        Settle {
            clearing_price: 1_000_000,
            recipient: crate::shield::join::address_from_u64(0xBEEF),
        },
        None,
    );

    assert!(
        satisfies(&js.wired, &js.witness),
        "a full unshield of two notes does not satisfy its own constraints"
    );
}

/// Conservation is what makes the unshield an unshield, so break it and the
/// circuit must refuse. One wei more claimed publicly than the notes hold.
#[test]
fn an_unshield_that_claims_more_than_the_notes_hold_is_refused() {
    let sks = [secret(41), secret(42)];
    let ins = [owned(sks[0], 100, 1_995_000), owned(sks[1], 200, 1_995_000)];
    let outs = [plain(300, 0), plain(400, 0)];

    let js = crate::shield::join::join_split_at(
        MINIMAL,
        [
            Spend {
                note: &ins[0],
                sk: sks[0],
            },
            Spend {
                note: &ins[1],
                sk: sks[1],
            },
        ],
        [&outs[0], &outs[1]],
        1_995_000 + 1_995_000 + 1,
        0,
        Break::None,
        Settle {
            clearing_price: 1_000_000,
            recipient: crate::shield::join::address_from_u64(0xBEEF),
        },
        None,
    );

    assert!(
        !satisfies(&js.wired, &js.witness),
        "the circuit accepted an unshield claiming more than its inputs"
    );
    let _ = join_split;
}
