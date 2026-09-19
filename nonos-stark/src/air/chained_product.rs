// NONOS Operating System (AGPL-3.0-or-later)
//! One grand product over every wired column, split into degree-bounded steps.
//!
//! Groups exist to bound the degree: a product over `k` columns is degree
//! `k + 1` and the budget is thirteen. The price is a sigma column per column
//! per group, and on the settlement outer that is 2,025 columns for wiring
//! over 368.
//!
//! Chaining buys the same bound differently. A product over `K` columns is
//! `B = ceil(K / BLOCK)` steps carried through intermediate columns:
//!
//!     t_0(r)     * den_0(r)     = z(r)       * num_0(r)
//!     t_m(r)     * den_m(r)     = t_{m-1}(r) * num_m(r)        1 <= m < B-1
//!     z(r + 1)   * den_{B-1}(r) = t_{B-2}(r) * num_{B-1}(r)
//!
//! Multiply them and the wide identity falls out, so one sigma column per
//! column is enough.
//!
//! Constraint algebra only. The wiring comes from
//! `recursion_assembly::groups::single`, the column placement from the fused
//! AIR.

use super::super::field::{Felt, Fp};
use alloc::vec::Vec;

/// Columns folded into one step, so a lane is degree `BLOCK + 1`. Eight is
/// what the packed groups already carry.
pub const BLOCK: usize = 8;

/// How many accumulator columns a permutation over `k` columns needs.
pub fn blocks(k: usize) -> usize {
    k.div_ceil(BLOCK).max(1)
}

/// One step's numerator and denominator over the cells it covers.
///
/// The identity `row * k + j` is passed in rather than computed: the row is a
/// periodic column the fused AIR already carries, and deriving it twice is two
/// places to get it wrong.
pub fn step<F: Felt>(values: &[F], identity: &[F], sigma: &[F], beta: Fp, gamma: Fp) -> (F, F) {
    let b = F::from_base(beta);
    let g = F::from_base(gamma);
    let mut num = F::ONE;
    let mut den = F::ONE;
    for ((v, id), sg) in values.iter().zip(identity.iter()).zip(sigma.iter()) {
        num = num * (*v + b * *id + g);
        den = den * (*v + b * *sg + g);
    }
    (num, den)
}

/// The `B` lanes of the chained argument at one row.
///
/// `acc[B-1]` is the running product; the columns below it hold this row's
/// partials. Every lane must be zero.
///
/// Only the last lane crosses the window, so only it can hold a column still,
/// and off the argument's rows that is what it does. The intermediate lanes
/// relate two cells of one row and simply go quiet there, which is fine: the
/// partials feed nothing outside this argument and the product they feed is
/// held.
pub fn lanes<F: Felt>(steps: &[(F, F)], acc: &[F], acc_next: &[F], sel: F) -> Vec<F> {
    let b = steps.len();
    debug_assert_eq!(acc.len(), b, "one accumulator column per step");
    debug_assert_eq!(acc_next.len(), b, "the next row carries the same columns");
    let z = acc[b - 1];
    let mut out = Vec::with_capacity(b);
    for (m, (num, den)) in steps.iter().enumerate() {
        /*
         * Step 0 picks up the product as it stood at the start of the row.
         * Later steps continue from the one before. The last writes the next
         * row's product.
         */
        if m + 1 == b {
            let prev = if b == 1 { z } else { acc[b - 2] };
            let advance = acc_next[b - 1] * *den - prev * *num;
            let hold = acc_next[b - 1] - z;
            out.push(sel * advance + (F::ONE - sel) * hold);
        } else {
            let prev = if m == 0 { z } else { acc[m - 1] };
            out.push(sel * (acc[m] * *den - prev * *num));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::Fp;

    fn f(x: u64) -> Fp {
        Fp::from_u64(x)
    }

    /// The chained lanes are satisfied exactly when the wide product is, which
    /// is the whole case for splitting it.
    #[test]
    fn the_chain_is_the_wide_product() {
        let (beta, gamma) = (f(5), f(7));
        let k = 20usize;
        let b = blocks(k);
        assert_eq!(b, 3, "twenty columns in steps of eight is three");

        // A row of cells, their slot identities, and a permutation that is a
        // single cycle over the first three slots and fixes the rest.
        let values: Vec<Fp> = (0..k).map(|j| f(100 + j as u64)).collect();
        let identity: Vec<Fp> = (0..k).map(|j| f(j as u64)).collect();
        let mut sigma = identity.clone();
        sigma[0] = f(1);
        sigma[1] = f(2);
        sigma[2] = f(0);

        let steps: Vec<(Fp, Fp)> = (0..b)
            .map(|m| {
                let lo = m * BLOCK;
                let hi = core::cmp::min(lo + BLOCK, k);
                step(
                    &values[lo..hi],
                    &identity[lo..hi],
                    &sigma[lo..hi],
                    beta,
                    gamma,
                )
            })
            .collect();

        // An honest accumulator: start from some z, fold each step in turn.
        let z = f(3);
        let mut acc = alloc::vec![Fp::ZERO; b];
        let mut running = z;
        for (m, (num, den)) in steps.iter().enumerate() {
            running = running * *num * den.inv();
            if m + 1 < b {
                acc[m] = running;
            }
        }
        acc[b - 1] = z;
        let mut acc_next = acc.clone();
        acc_next[b - 1] = running;

        for (m, lane) in lanes(&steps, &acc, &acc_next, Fp::ONE)
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                lane,
                Fp::ZERO,
                "chained lane {m} does not vanish on an honest row"
            );
        }

        // And the identity it imposes is the unsplit one.
        let (mut num_all, mut den_all) = (Fp::ONE, Fp::ONE);
        for (num, den) in &steps {
            num_all = num_all * *num;
            den_all = den_all * *den;
        }
        assert_eq!(
            running * den_all,
            z * num_all,
            "the chain does not impose the wide product's identity"
        );
    }

    /// A tampered accumulator must break a lane, or the split admits what the
    /// wide form refuses.
    #[test]
    fn a_wrong_intermediate_breaks_a_lane() {
        let (beta, gamma) = (f(5), f(7));
        let k = 20usize;
        let b = blocks(k);
        let values: Vec<Fp> = (0..k).map(|j| f(100 + j as u64)).collect();
        let identity: Vec<Fp> = (0..k).map(|j| f(j as u64)).collect();
        let mut sigma = identity.clone();
        sigma.swap(0, 1);

        let steps: Vec<(Fp, Fp)> = (0..b)
            .map(|m| {
                let lo = m * BLOCK;
                let hi = core::cmp::min(lo + BLOCK, k);
                step(
                    &values[lo..hi],
                    &identity[lo..hi],
                    &sigma[lo..hi],
                    beta,
                    gamma,
                )
            })
            .collect();

        let z = f(3);
        let mut acc = alloc::vec![Fp::ZERO; b];
        let mut running = z;
        for (m, (num, den)) in steps.iter().enumerate() {
            running = running * *num * den.inv();
            if m + 1 < b {
                acc[m] = running;
            }
        }
        acc[b - 1] = z;
        let mut acc_next = acc.clone();
        acc_next[b - 1] = running;

        // Nudge one intermediate. Some lane must notice.
        acc[0] = acc[0] + Fp::ONE;
        let broken = lanes(&steps, &acc, &acc_next, Fp::ONE);
        assert!(
            broken.iter().any(|l| *l != Fp::ZERO),
            "a tampered intermediate satisfied every lane"
        );
    }

    /// Off the argument's rows the product is pinned to hold, whatever the
    /// intermediates do.
    #[test]
    fn the_selector_pins_the_running_product() {
        let steps = alloc::vec![(f(11), f(13)), (f(17), f(19))];
        let acc = alloc::vec![f(2), f(3)];

        // Held, and with a junk intermediate on the next row: still silent.
        let held = alloc::vec![f(999), f(3)];
        for (m, lane) in lanes(&steps, &acc, &held, Fp::ZERO).into_iter().enumerate() {
            assert_eq!(lane, Fp::ZERO, "lane {m} objected to a held product");
        }

        // The product itself moving must be caught.
        let moved = alloc::vec![f(2), f(4)];
        assert!(
            lanes(&steps, &acc, &moved, Fp::ZERO)
                .iter()
                .any(|l| *l != Fp::ZERO),
            "the running product moved while the selector was off and no lane objected"
        );
    }
}
