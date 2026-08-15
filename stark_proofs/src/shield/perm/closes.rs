// NONOS Operating System (AGPL-3.0-or-later)

use crate::crypto::stark::air::{Cell, WirePermutation, WiredPermutationArg};
use crate::crypto::stark::field::Fp;

fn arg(rows: usize, width: usize, class: &[Cell]) -> WiredPermutationArg {
    let mut p = WirePermutation::identity(rows, width);
    p.add_class(class);
    WiredPermutationArg::from_permutation(&p, 2, Fp::from_u64(5), Fp::from_u64(7))
}

/// The product returns to one exactly when the class holds equal values. That is
/// the whole enforcement: a cycle cancels only if every cell in it carries the
/// same value.
#[test]
fn the_product_closes_on_a_satisfied_class() {
    let class = [Cell { row: 0, col: 0 }, Cell { row: 2, col: 1 }];
    let a = arg(4, 2, &class);
    let mut cells = alloc::vec![Fp::from_u64(9); 8];
    cells[0] = Fp::from_u64(42);
    cells[2 * 2 + 1] = Fp::from_u64(42);
    let z = a.trace(&cells);
    assert_eq!(z[0], Fp::ONE);
    let last = z[z.len() - 1];
    let mut num = Fp::ONE;
    let mut den = Fp::ONE;
    let r = z.len() - 1;
    for j in 0..2 {
        let v = cells[r * 2 + j];
        num = num * (v + a.beta * a.id[j][r] + a.gamma);
        den = den * (v + a.beta * a.sigma[j][r] + a.gamma);
    }
    assert_eq!(last * num * den.inv(), Fp::ONE, "a satisfied class must close");
}

/// And does not close when they differ, which is what makes it a binding rather
/// than bookkeeping.
#[test]
fn the_product_does_not_close_on_a_violated_class() {
    let class = [Cell { row: 0, col: 0 }, Cell { row: 2, col: 1 }];
    let a = arg(4, 2, &class);
    let mut cells = alloc::vec![Fp::from_u64(9); 8];
    cells[0] = Fp::from_u64(42);
    cells[2 * 2 + 1] = Fp::from_u64(43);
    let z = a.trace(&cells);
    let r = z.len() - 1;
    let mut num = Fp::ONE;
    let mut den = Fp::ONE;
    for j in 0..2 {
        let v = cells[r * 2 + j];
        num = num * (v + a.beta * a.id[j][r] + a.gamma);
        den = den * (v + a.beta * a.sigma[j][r] + a.gamma);
    }
    assert_ne!(z[r] * num * den.inv(), Fp::ONE, "a violated class must not close");
}
