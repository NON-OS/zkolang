// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{classes_are_layable, Cell};

fn cell(row: usize, col: usize) -> Cell {
    Cell { row, col }
}

/// Sharing one cell merges the two cycles, which is what the shield relies on: a
/// commitment is shared by both memberships and by the nullifier absorbing it.
#[test]
fn classes_sharing_one_cell_are_layable() {
    let a = alloc::vec![cell(0, 0), cell(1, 0)];
    let b = alloc::vec![cell(1, 0), cell(2, 0)];
    assert!(classes_are_layable(&[a, b]));
}

/// Sharing two closes the chain early and leaves a cell pointing at itself, so a
/// binding that held is gone while the product still closes.
#[test]
fn a_class_over_cells_already_joined_is_not() {
    let a = alloc::vec![cell(0, 0), cell(1, 0), cell(2, 0)];
    let b = alloc::vec![cell(0, 0), cell(2, 0)];
    assert!(!classes_are_layable(&[a, b]));
}

/// The degenerate case of the same: a class laid twice.
#[test]
fn the_same_class_twice_is_not() {
    let a = alloc::vec![cell(3, 1), cell(4, 1)];
    assert!(!classes_are_layable(&[a.clone(), a]));
}
