// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{classes_are_layable, Cell, RATE, WIDTH};
use crate::shield::agg::{cells, effect_classes, LANES};
use crate::shield::join::publics::WORDS;

const L: usize = 32;

/// Sixteen lanes, one class each, and the set lays: nothing shares a cell, so no
/// binding is silently dropped when the classes go onto the permutation.
#[test]
fn the_effect_binding_lays() {
    let g = effect_classes(L, 4096, 0);
    assert_eq!(g.len(), LANES);
    assert!(classes_are_layable(&g));
}

/// Every class reaches the injection column. A class landing on a state lane
/// would tie the effect to the sponge mid flight, which tracks the publics
/// without being them.
#[test]
fn every_class_reaches_the_absorbed_cell() {
    let g = effect_classes(L, 4096, 0);
    for k in &g {
        assert!(
            k.iter().any(|c| c.col == WIDTH),
            "no class member on the injection column"
        );
        assert!(
            k.iter().any(|c: &Cell| c.row >= 4096),
            "the effect side went unbound"
        );
    }
}

/// Two children of one node bind to different words, so a node cannot tie both
/// effects to one child's publics and compose it twice.
#[test]
fn two_children_bind_to_different_cells() {
    let (a, b) = (
        effect_classes(L, 4096, 0),
        effect_classes(L, 4096 + LANES, WORDS),
    );
    let (ca, cb) = (cells(&a), cells(&b));
    assert!(ca.iter().all(|x| !cb.contains(x)));
    let mut both = a;
    both.extend(b);
    assert!(classes_are_layable(&both));
}
