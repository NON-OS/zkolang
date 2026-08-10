// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{classes_are_disjoint, Cell};

/// Necessary, not sufficient. It proves classes do not collide; enforcement is
/// what the per binding forgeries answer, and a green check here must never
/// shorten that migration.
#[test]
fn overlapping_classes_are_rejected() {
    let a = alloc::vec![Cell { row: 0, col: 0 }, Cell { row: 1, col: 0 }];
    let b = alloc::vec![Cell { row: 1, col: 0 }, Cell { row: 2, col: 0 }];
    assert!(!classes_are_disjoint(&[a.clone(), b]));
    let c = alloc::vec![Cell { row: 3, col: 0 }, Cell { row: 4, col: 0 }];
    assert!(classes_are_disjoint(&[a, c]));
}
