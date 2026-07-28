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

    let mut len = 2usize;
    while len <= n {
        let half = len / 2;
        // The twiddle factors for this level, `w_len^0 .. w_len^{half-1}`. They are
        // the same for every block, so build them once and index rather than
        // regenerating the sequence with a running multiply inside each block.
        let w_len = omega.pow((n / len) as u64);
        let mut twiddles = Vec::with_capacity(half);
        let mut w = Fp::ONE;
        for _ in 0..half {
            twiddles.push(w);
            w = w * w_len;
        }
        let mut start = 0usize;
        while start < n {
            for j in 0..half {
                let u = a[start + j];
                let v = a[start + j + half] * twiddles[j];
                a[start + j] = u + v;
                a[start + j + half] = u - v;
            }
            start += len;
        }
        len <<= 1;
    }
    a
}

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
    use super::{intt, ntt};
    use crate::field::Fp;
    use crate::fri::root_of_unity;
    use alloc::vec::Vec;

    // The definition the fast transform must meet: out[i] = sum_j coeffs[j] * omega^(i j).
    fn naive(coeffs: &[Fp], omega: Fp) -> Vec<Fp> {
        (0..coeffs.len())
            .map(|i| {
                let wi = omega.pow(i as u64);
                let mut acc = Fp::ZERO;
                let mut wij = Fp::ONE;
                for &c in coeffs {
                    acc = acc + c * wij;
                    wij = wij * wi;
                }
                acc
            })
            .collect()
    }

    #[test]
    fn ntt_matches_the_definition_and_inverts() {
        for log_n in 1..=9u32 {
            let n = 1usize << log_n;
            let omega = root_of_unity(log_n);
            let coeffs: Vec<Fp> = (0..n)
                .map(|i| Fp::from_u64((i as u64).wrapping_mul(2_654_435_761).wrapping_add(7)))
                .collect();
            let fast = ntt(&coeffs, omega);
            assert_eq!(
                fast,
                naive(&coeffs, omega),
                "ntt disagrees with the DFT, log_n={log_n}"
            );
            assert_eq!(
                intt(&fast, omega),
                coeffs,
                "intt does not invert ntt, log_n={log_n}"
            );
        }
    }
}
