// NONOS Operating System (AGPL-3.0-or-later)

use super::satisfies::satisfies;
use super::scenario::{balanced, balanced_flip};
use crate::shield::join::publics::{
    ASSET_ID, ASSOC_ROOT, FEE, NF0, NF1, NOTE_ROOT, OUT_CM0, OUT_CM1, PUBLIC_AMOUNT, WORDS,
};
use crate::shield::key::Break;

/// Rows the circuit computes. Each is copy constrained to the cell producing it,
/// so flipping the claim contradicts the trace.
fn derived() -> alloc::vec::Vec<usize> {
    let mut v = alloc::vec::Vec::new();
    for base in [NOTE_ROOT, ASSOC_ROOT, NF0, NF1, OUT_CM0, OUT_CM1] {
        v.extend(base..base + 4);
    }
    v.extend([PUBLIC_AMOUNT, FEE, ASSET_ID]);
    v
}

/// Positive binding. A flip rejecting shows a word is constrained to something;
/// the honest run satisfying the same trace shows it is constrained to the cell
/// that computes it.
#[test]
fn every_derived_word_is_bound_to_its_computed_cell() {
    let honest = balanced(Break::None);
    assert!(satisfies(&honest.wired, &honest.witness));
    assert_eq!(honest.intent.len(), WORDS);

    for i in derived() {
        let js = balanced_flip(Break::None, Some(i));
        assert!(!satisfies(&js.wired, &js.witness), "derived word {i} is not bound");
    }
}
