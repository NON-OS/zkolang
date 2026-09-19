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

//! The DEEP polynomial for the preprocessed path. It carries the plain path's trace and
//! composition quotients and adds one quotient per periodic column, each subtracting the
//! claimed periodic value at the out-of-domain point and dividing by its vanishing factor. The
//! extra quotients are what tie the baked schedule's opened rows to the claims the composition
//! consumed, so the sidecar is bound rather than trusted.

use super::super::super::field::{Fp, Fp2};
use super::super::prove_ext::{batch_inv, extend, Domain, BLOCK};
use alloc::vec::Vec;

/// The preprocessed DEEP polynomial: the trace and composition quotients of the
/// plain form, plus one quotient per periodic column against its claimed value
/// at z, which is what lets the verifier hold the periodic root as a constant
/// instead of recomputing the schedule. Coset-walked; both column families are
/// extended here rather than held.
#[allow(clippy::too_many_arguments)]
pub(in crate::air) fn over_domain(
    d: &Domain,
    trace: &[Vec<Fp>],
    periodic: &[Vec<Fp>],
    comp_d: &[Fp2],
    ood_frame: &[Fp2],
    periodic_z: &[Fp2],
    comp_z: Fp2,
    z: Fp2,
    deep_coeffs: &[Fp2],
) -> Vec<Fp2> {
    let zks: Vec<Fp2> = (0..d.window)
        .map(|k| z * Fp2::from_base(d.g.pow(k as u64)))
        .collect();
    let e = deep_coeffs[d.width * d.window];

    /*
     * Every quotient family shares a divisor, so the fold distributes:
     *
     *   sum_j c_j (v_j(x) - claim_j) / (x - z_k)
     *     = (sum_j c_j v_j(x) - sum_j c_j claim_j) / (x - z_k)
     *
     * The second sum does not depend on x. It is taken once here, one value
     * per window row for the trace and one for the periodic claims, and every
     * point then pays one base-scaled product per column instead of an
     * extension subtraction and two extension products, and one division per
     * family instead of one per column. Same field elements, since the
     * identity is exact; the DEEP polynomial and everything committed after
     * it are unchanged. On the deployed outer this is 4,148 terms a point,
     * across sixty seven million points.
     */
    let claim_sums: Vec<Fp2> = (0..d.window)
        .map(|k| {
            let mut s = Fp2::ZERO;
            for col in 0..d.width {
                s = s + deep_coeffs[k * d.width + col] * ood_frame[k * d.width + col];
            }
            s
        })
        .collect();
    let periodic_coeffs = &deep_coeffs[d.width * d.window + 1..];
    let mut periodic_claim_sum = Fp2::ZERO;
    for (pc, claim) in periodic_coeffs.iter().zip(periodic_z.iter()) {
        periodic_claim_sum = periodic_claim_sum + *pc * *claim;
    }

    let width = d.width;
    let mut deep_d = alloc::vec![Fp2::ZERO; d.n];
    for c in 0..d.blowup {
        let cols = extend(trace, d, c);
        let per = extend(periodic, d, c);
        let shift_c = d.coset_shift(c);
        let blocks = d.t.div_ceil(BLOCK);
        let parts = crate::par::map_index(blocks, |b| {
            let (lo, hi) = (b * BLOCK, ((b + 1) * BLOCK).min(d.t));
            // Every denominator in the block at once; the (x - z) inverse is
            // shared by the composition and every periodic quotient.
            let stride = zks.len() + 1;
            let mut dens: Vec<Fp2> = Vec::with_capacity((hi - lo) * stride);
            let mut x = shift_c * d.sub.pow(lo as u64);
            for _ in lo..hi {
                let xe = Fp2::from_base(x);
                for zk in &zks {
                    dens.push(xe - *zk);
                }
                dens.push(xe - z);
                x = x * d.sub;
            }
            let invs = batch_inv(&dens);

            let mut out: Vec<Fp2> = Vec::with_capacity(hi - lo);
            for i in lo..hi {
                let j = c + d.blowup * i;
                let base = (i - lo) * stride;
                let mut acc = Fp2::ZERO;
                for k in 0..zks.len() {
                    let mut row = Fp2::ZERO;
                    let dc = &deep_coeffs[k * width..(k + 1) * width];
                    for (coeff, column) in dc.iter().zip(cols.iter()) {
                        row = row + coeff.mul_base(column[i]);
                    }
                    acc = acc + (row - claim_sums[k]) * invs[base + k];
                }
                let inv_x_z = invs[base + zks.len()];
                acc = acc + e * ((comp_d[j] - comp_z) * inv_x_z);
                let mut periodic_row = Fp2::ZERO;
                for (pc, pd) in periodic_coeffs.iter().zip(per.iter()) {
                    periodic_row = periodic_row + pc.mul_base(pd[i]);
                }
                acc = acc + (periodic_row - periodic_claim_sum) * inv_x_z;
                out.push(acc);
            }
            out
        });
        for (b, part) in parts.into_iter().enumerate() {
            for (k, v) in part.into_iter().enumerate() {
                deep_d[c + d.blowup * (b * BLOCK + k)] = v;
            }
        }
    }
    deep_d
}
