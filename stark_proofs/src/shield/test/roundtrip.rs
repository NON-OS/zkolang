// NONOS Operating System (AGPL-3.0-or-later)

use super::scenario::balanced;
use crate::crypto::stark::air::{stark_prove_ext, stark_verify_ext};
use crate::shield::key::Break;

/// A real proof of a private transfer, produced and verified. Not a witness
/// satisfaction check: the prover runs, FRI commits, the verifier accepts.
#[test]
#[ignore]
fn a_private_transfer_proves_and_verifies() {
    let js = balanced(Break::None);
    let proof = stark_prove_ext(&js.wired, &js.witness, 32, 8);
    assert!(stark_verify_ext(&js.wired, &proof, 32, 8), "the transfer did not verify");
}

/// And a spend of a note the spender does not own has no proof at all.
#[test]
#[ignore]
fn a_stolen_note_has_no_proof() {
    let js = balanced(Break::ForeignNote);
    let proof = stark_prove_ext(&js.wired, &js.witness, 32, 8);
    assert!(!stark_verify_ext(&js.wired, &proof, 32, 8), "a stolen note verified");
}
