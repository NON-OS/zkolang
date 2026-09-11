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

//! Zero-knowledge blinding of a trace column. A plain STARK is not hiding: a FRI
//! query opens the trace at points off the domain, and those openings are a
//! deterministic function of the witness. Blinding removes that leak. Each column
//! polynomial `f` is replaced by `f + r * Z_H`, where `Z_H = x^t - 1` is the
//! vanishing polynomial of the trace domain and `r` is fresh randomness.
//!
//! On the trace domain, where `x^t = 1`, `Z_H` is zero, so the blinded polynomial
//! takes the same value as `f` at every row. The trace the constraints see is
//! unchanged, and a satisfying trace stays satisfying, so blinding costs no
//! soundness. Off the domain, where the queries land, the value is shifted by
//! `r(x) * Z_H(x)`; with `deg r >= n_queries`, the shifts at the query points are
//! jointly uniform, so the openings reveal nothing about the witness. This is the
//! hiding a zero-knowledge STARK adds over the plain one.

use super::super::field::Fp;
use alloc::vec::Vec;

/// `coeffs + r * (x^t - 1)`, low degree first. `coeffs` is a trace column, degree
/// below `t`; `r` is the blinding polynomial, `deg r + 1` coefficients. The result
/// has degree `t + deg r`, unchanged on the trace domain and randomized off it.
pub fn blind_coeffs(coeffs: &[Fp], t: usize, r: &[Fp]) -> Vec<Fp> {
    let mut out = coeffs.to_vec();
    if out.len() < t {
        out.resize(t, Fp::ZERO);
    }
    out.resize(t + r.len(), Fp::ZERO);
    for (j, &rj) in r.iter().enumerate() {
        // f + r * (x^t - 1): subtract r at the low end, add r shifted by x^t.
        out[j] = out[j] - rj;
        out[t + j] = out[t + j] + rj;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fri::root_of_unity;
    use crate::poly::eval;

    /// The property the whole scheme rests on: the blinded column agrees with the
    /// original at every trace row, so no constraint sees the blinding, while off
    /// the domain the value genuinely moves, which is where the queries open and
    /// the hiding lives.
    #[test]
    fn blinding_is_invisible_on_the_trace_domain() {
        let log_t = 4u32;
        let t = 1usize << log_t;
        let g = root_of_unity(log_t);

        let coeffs: Vec<Fp> = (0..t).map(|i| Fp::from_u64((7 * i as u64 + 3) % 101)).collect();
        // Blinding degree 8, standing in for a query count.
        let r: Vec<Fp> = (0..9).map(|i| Fp::from_u64((13 * i as u64 + 5) % 97)).collect();
        let blinded = blind_coeffs(&coeffs, t, &r);

        // Every trace row: the blinded value equals the original.
        let mut x = Fp::ONE;
        for _ in 0..t {
            assert_eq!(eval(&blinded, x), eval(&coeffs, x), "blinding changed a trace value");
            x = x * g;
        }

        // Off the domain, the value moves by exactly r(x) * (x^t - 1).
        let off = Fp::from_u64(7);
        let zh = off.pow(t as u64) - Fp::ONE;
        assert_eq!(eval(&blinded, off) - eval(&coeffs, off), eval(&r, off) * zh);
        assert_ne!(eval(&blinded, off), eval(&coeffs, off), "blinding had no off-domain effect");
    }
}
