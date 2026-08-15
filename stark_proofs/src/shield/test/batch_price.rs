// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::intent_at_price;
use crate::shield::batch::batch;

/// A batch clears at one price, so fills cannot be priced against each other and
/// no order carries its own price. Uniformity is the property; the negative is
/// the settler pricing one intent differently.
#[test]
fn a_batch_clears_at_one_price() {
    let b = batch(&[intent_at_price(1_000_000), intent_at_price(1_000_000)]);
    assert!(satisfies(&b.wired, &b.witness));
}

#[test]
fn an_intent_priced_apart_from_the_batch_rejects() {
    let b = batch(&[intent_at_price(1_000_000), intent_at_price(1_000_001)]);
    assert!(!satisfies(&b.wired, &b.witness));
}
