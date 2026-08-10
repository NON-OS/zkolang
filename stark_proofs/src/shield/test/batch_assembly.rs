// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::intents::intent;
use crate::shield::batch::assemble;
use crate::shield::join::publics::CLEARING_PRICE;

/// Two whole join splits under one proof: every note, membership, key hierarchy
/// and balance in the same trace, each intent binding its own words, and the
/// price tied across both.
#[test]
fn a_two_intent_batch_settles_under_one_proof() {
    let b = assemble(alloc::vec![intent(1, 1_000_000, None), intent(2, 1_000_000, None)]);
    assert!(satisfies(&b.wired, &b.witness));
    assert_eq!(b.intents.len(), 2);
}

/// A later intent's derived word is still bound: the batch does not dilute the
/// per intent bindings.
#[test]
fn a_tampered_word_in_the_second_intent_rejects() {
    let b = assemble(alloc::vec![intent(1, 1_000_000, None), intent(2, 1_000_000, Some(0))]);
    assert!(!satisfies(&b.wired, &b.witness));
}

#[test]
fn intents_priced_apart_reject() {
    let b = assemble(alloc::vec![intent(1, 1_000_000, None), intent(2, 1_000_001, None)]);
    assert!(!satisfies(&b.wired, &b.witness));
    let _ = CLEARING_PRICE;
}
