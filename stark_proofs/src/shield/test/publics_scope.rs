// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced_flip;
use crate::shield::join::publics::{CLEARING_PRICE, RECIPIENT};
use crate::shield::key::Break;

/// The settlement terms are inputs to the statement, not outputs of it, so no
/// constraint can tie them to a computed cell and flipping one is satisfiable
/// here. They are bound where such terms are always bound: absorbed into the
/// transcript, so a proof is void for any other tuple.
///
/// The clearing price is declared per intent; uniformity across a batch is a
/// separate constraint and lives with the batch.
#[test]
#[ignore]
fn settlement_terms_are_transcript_bound_not_constraint_bound() {
    for i in [CLEARING_PRICE, RECIPIENT] {
        let js = balanced_flip(Break::None, Some(i));
        assert!(
            satisfies(&js.wired, &js.witness),
            "word {i} became constraint bound; fold it into the derived set"
        );
    }
}
