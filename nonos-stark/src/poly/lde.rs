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

//! The low-degree extension by transform: interpolate values on the trace
//! subgroup, then evaluate the same polynomial on a larger coset. This is the
//! per-column extension the prover runs, in O(n log n).

use super::super::field::Fp;
use super::ntt::{intt, ntt};
use alloc::vec::Vec;

/// Extend `values`, the evaluations of a polynomial on the size-`values.len()`
/// subgroup `{trace_gen^i}`, onto the coset `shift * {coset_gen^i}` of size
/// `target_len`. `trace_gen` and `coset_gen` are primitive roots of unity of the
/// respective orders. The result equals evaluating the interpolating polynomial
/// at each coset point, computed by transform rather than point by point.
pub fn lde(values: &[Fp], trace_gen: Fp, shift: Fp, coset_gen: Fp, target_len: usize) -> Vec<Fp> {
    lde_from_coeffs(&intt(values, trace_gen), shift, coset_gen, target_len)
}

/// The same extension, from coefficients already in hand. A caller evaluating one
/// polynomial over many cosets interpolates once and calls this per coset, rather
/// than paying the interpolation again for every one of them.
pub fn lde_from_coeffs(coeffs: &[Fp], shift: Fp, coset_gen: Fp, target_len: usize) -> Vec<Fp> {
    // Evaluate `P(x) = sum coeffs[j] x^j` on the coset `shift * {coset_gen^i}` for
    // `i` in `0..target_len`. On that coset `coset_gen` has order `target_len`, so
    // at `x = shift * coset_gen^i` the term `x^j` is `shift^j * coset_gen^(i * (j
    // mod target_len))`: coefficient `j` lands on residue `j mod target_len` with
    // weight `shift^j`. Folding the high coefficients onto their residue, rather
    // than dropping them, is what evaluates a polynomial whose degree reaches or
    // exceeds `target_len` correctly, which a blinded trace column `f + r * Z_H`
    // (degree `t + deg r`) is. Below `target_len` nothing folds and this is the
    // plain scale-and-transform, identical to the coefficient.
    let mut c = alloc::vec![Fp::ZERO; target_len];
    let mut s = Fp::ONE;
    for (j, &cj) in coeffs.iter().enumerate() {
        let k = j % target_len;
        c[k] = c[k] + cj * s;
        s = s * shift;
    }
    ntt(&c, coset_gen)
}
