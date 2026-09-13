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
use super::super::par::for_each_chunk_mut;
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
        // A primitive len-th root of unity.
        let w_len = omega.pow((n / len) as u64);
        let half = len / 2;
        /*
         * One stage's butterflies fall into blocks of `len` that touch nothing
         * outside themselves, so the blocks go wide. Each block walks its own
         * twiddle up from one, exactly as the serial loop did, so the arithmetic,
         * and with it the proof, is the same whichever way the crate is built.
         */
        for_each_chunk_mut(&mut a, len, |block| {
            let mut w = Fp::ONE;
            for j in 0..half {
                let u = block[j];
                let v = block[j + half] * w;
                block[j] = u + v;
                block[j + half] = u - v;
                w = w * w_len;
            }
        });
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
