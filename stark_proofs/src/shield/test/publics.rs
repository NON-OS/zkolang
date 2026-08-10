// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::{balanced, balanced_flip};
use crate::shield::join::publics::WORDS;
use crate::shield::key::Break;

/// Positive binding on every word. A tamper rejecting shows a word is
/// constrained to something; this shows it is constrained to the cell that
/// computes it, since the honest run pins the same trace to the true value.
#[test]
fn every_public_word_is_bound_to_its_computed_cell() {
    let honest = balanced(Break::None);
    assert!(satisfies(&honest.wired, &honest.witness));
    assert_eq!(honest.intent.len(), WORDS);

    for i in 0..WORDS {
        let js = balanced_flip(Break::None, Some(i));
        assert!(!satisfies(&js.wired, &js.witness), "public word {i} is not bound");
    }
}
