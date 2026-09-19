// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The number-theoretic transform: evaluate a polynomial on a multiplicative
//! subgroup, and its inverse, in O(n log n) instead of O(n^2). This is what lets
//! the prover extend a trace to a large domain at scale. `omega` must be a
//! primitive `n`-th root of unity, where `n` is the length, a power of two.

use super::super::field::Fp;
use alloc::vec::Vec;

/// Reorder `a` into bit-reversed index order in place.
fn bit_reverse(a: &mut [Fp]) {
    let n = a.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
}

/// Evaluate `coeffs` (low degree first) on `{omega^0, ..., omega^{n-1}}` by the
/// iterative radix-2 transform. Returns the `n` evaluations in natural order.
pub fn ntt(coeffs: &[Fp], omega: Fp) -> Vec<Fp> {
    let n = coeffs.len();
    let mut a = coeffs.to_vec();
    if n <= 1 {
        return a;
    }
    bit_reverse(&mut a);

    /*
     * Serial, on purpose. Every caller runs one transform per column inside a
     * map that is already parallel across columns, so a transform that also
     * split its own butterflies into tasks was handing the scheduler a task
     * per pair at the first stage: a quarter of a million tasks for a 2^19
     * column, nineteen times over, for every one of thousands of columns per
     * coset. Sampled on the box, that scheduling was more of the DEEP phase
     * than the DEEP arithmetic. The butterflies themselves are unchanged.
     *
     * The twiddles for a stage are the same in every block of that stage, so
     * they are walked up from one once, into a table, rather than once per
     * block: half the multiplications of the stage, and the same values,
     * because the table is the same running product the blocks each ran.
     */
    /*
     * Twiddles are walked multiplicatively, never tabulated.
     *
     * A table of `omega^k` looks like the obvious trade, one multiplication
     * for one load, and it is a loss here. At `n = 2^18` the table is a
     * megabyte, the middle stages read it at a stride of kilobytes, and
     * forty four threads each want their own: the tables alone exceed the
     * shared cache and the transform goes from arithmetic-bound to
     * memory-bound. Measured on the box, that cost more than the
     * multiplications it removed. A running product is two registers.
     */
    let mut len = 2usize;
    while len <= n {
        let w_len = omega.pow((n / len) as u64);
        let half = len / 2;
        if half < SHORT_STAGE {
            /*
             * Short blocks, many of them. Iterating blocks outermost paid a
             * slice split and two fresh iterators per pair at exactly the
             * stages that hold most of the pairs. With the twiddle outermost
             * the walk is one multiplication per stage step and the inner
             * loop is long. Same pairs, same twiddles, same order of
             * arithmetic within a pair.
             */
            let mut tw = Fp::ONE;
            for j in 0..half {
                let mut start = j;
                while start < n {
                    let u = a[start];
                    let v = a[start + half] * tw;
                    a[start] = u + v;
                    a[start + half] = u - v;
                    start += len;
                }
                tw = tw * w_len;
            }
        } else {
            for block in a.chunks_exact_mut(len) {
                let (lo, hi) = block.split_at_mut(half);
                let mut tw = Fp::ONE;
                for (x, y) in lo.iter_mut().zip(hi.iter_mut()) {
                    let u = *x;
                    let v = *y * tw;
                    *x = u + v;
                    *y = u - v;
                    tw = tw * w_len;
                }
            }
        }
        len <<= 1;
    }
    a
}

/// Below this half-block length a stage runs twiddle-outer, block-inner.
const SHORT_STAGE: usize = 32;

/// Interpolate `evals` on `{omega^0, ..., omega^{n-1}}` back to coefficients: the
/// transform with the inverse root, scaled by `1/n`.
pub fn intt(evals: &[Fp], omega: Fp) -> Vec<Fp> {
    let n = evals.len();
    let mut a = ntt(evals, omega.inv());
    if n > 1 {
        let n_inv = Fp::from_u64(n as u64).inv();
        for x in a.iter_mut() {
            *x = *x * n_inv;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fri::root_of_unity;

    /// The transform inverts, at the sizes a settlement column has, and a
    /// bench line for the size the prover runs it at thousands of times per
    /// coset. Run the bench with `--ignored --nocapture`.
    #[test]
    fn the_inverse_transform_returns_the_coefficients() {
        for log_n in [1u32, 4, 10, 16] {
            let n = 1usize << log_n;
            let omega = root_of_unity(log_n);
            let coeffs: Vec<Fp> = (0..n as u64).map(|i| Fp::from_u64(i * 7 + 3)).collect();
            let back = intt(&ntt(&coeffs, omega), omega);
            assert_eq!(back, coeffs, "round trip moved at 2^{log_n}");
        }
    }

    #[test]
    #[ignore]
    #[cfg(feature = "parallel")]
    fn throughput() {
        let log_n = 19u32;
        let n = 1usize << log_n;
        let omega = root_of_unity(log_n);
        let coeffs: Vec<Fp> = (0..n as u64).map(|i| Fp::from_u64(i * 7 + 3)).collect();
        let reps = 8;
        let t = std::time::Instant::now();
        let mut acc = Fp::ZERO;
        for _ in 0..reps {
            acc = acc + ntt(&coeffs, omega)[1];
        }
        let s = t.elapsed().as_secs_f64() / reps as f64;
        std::println!("NTT 2^{log_n} {:.1} ms (acc {})", s * 1e3, acc.to_u64());
    }
}
