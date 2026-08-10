// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::balanced_flip;
use crate::shield::join::publics::{ASSOC_ROOT, CLEARING_PRICE, RECIPIENT};
use crate::shield::key::Break;

/// The settlement terms are inputs to the statement, not outputs of it, so no
/// constraint can tie them to a computed cell and flipping one is satisfiable
/// here. They are bound where such terms are always bound: absorbed into the
/// transcript, so a proof is void for any other tuple.
///
/// Two consequences worth stating rather than discovering. The association root
/// is only declared, so association set membership is not proven by this circuit.
/// The clearing price is only declared, so uniformity across a batch is not
/// proven either; that is the batch assembly's constraint.
#[test]
fn settlement_terms_are_transcript_bound_not_constraint_bound() {
    for i in [ASSOC_ROOT, CLEARING_PRICE, RECIPIENT] {
        let js = balanced_flip(Break::None, Some(i));
        assert!(
            satisfies(&js.wired, &js.witness),
            "word {i} became constraint bound; fold it into the derived set"
        );
    }
}
