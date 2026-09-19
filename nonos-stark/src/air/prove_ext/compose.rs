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

//! The composition polynomial over the evaluation domain, built one block of rows at a time.
//! At each point it evaluates the AIR's transition and boundary constraints on the trace
//! window, batches them under the transcript coefficients, and divides by the domain vanishing
//! polynomial, so the result is a genuine polynomial exactly when every constraint holds. The
//! block size trades memory for call overhead; the pass streams like the others, so the whole
//! composition is never held at once.

use super::super::super::field::{Fp, Fp2};
use super::super::composition::{compose_base_planned, ComposePlan};
use super::super::spec::AirExt;
use super::coset::extend;
use super::setup::Domain;
use alloc::vec::Vec;

/// Rows go to threads in blocks: a block allocates its window once, clears and
/// reuses it, and carries its own point forward multiplicatively.
pub(in crate::air) const BLOCK: usize = 1024;

/// The composition over the whole domain, walked coset by coset.
///
/// The window at position `j` reads `(j + k * blowup) % n`, which shares j's
/// residue mod blowup: a window never leaves its coset, it wraps to row
/// `(i + k) % t` of the same one. That wrap is what makes streaming exact, and
/// it is the only fact this function rests on.
///
/// Every point of the domain is a base field element and so is every trace
/// and periodic value at it, so the constraints are evaluated in the base
/// field and only the coefficient products lift. This pass used to lift every
/// input to the extension first and evaluate there, three base multiplications
/// for each one needed, and to take the vanishing inverse and a boundary-wide
/// batch inversion at every point when the first is a per coset constant and
/// the second has a distinct denominator per row, not per boundary. The
/// polynomial it produces is the same one; the values are the same field
/// elements reached by fewer operations.
pub(in crate::air) fn over_domain<A: AirExt>(
    air: &A,
    d: &Domain,
    trace: &[Vec<Fp>],
    periodic: &[Vec<Fp>],
    coeffs: &[Fp2],
) -> Vec<Fp2> {
    /*
     * The plan holds everything the composition needs that does not depend on
     * the point: the exemption points, the boundary list and its domain points.
     * Built once here rather than rebuilt inside the per point call, which is
     * what it used to be.
     */
    let plan = ComposePlan::new(air, d.g);
    let mut comp_d = alloc::vec![Fp2::ZERO; d.n];
    for c in 0..d.blowup {
        let cols = extend(trace, d, c);
        let per = extend(periodic, d, c);
        let shift_c = d.coset_shift(c);
        let z_h_inv = plan.vanishing_inv(shift_c);
        let blocks = d.t.div_ceil(BLOCK);
        let parts = crate::par::map_index(blocks, |b| {
            let (lo, hi) = (b * BLOCK, ((b + 1) * BLOCK).min(d.t));
            let mut window: Vec<Fp> = Vec::with_capacity(d.window * d.width);
            let mut periodic_i: Vec<Fp> = Vec::with_capacity(per.len());
            let mut out: Vec<Fp2> = Vec::with_capacity(hi - lo);
            // The denominator set and its prefixes, owned by the block so the
            // per point inversion allocates nothing.
            let mut den: Vec<Fp> = Vec::new();
            let mut prefix: Vec<Fp> = Vec::new();
            let mut x = shift_c * d.sub.pow(lo as u64);
            for i in lo..hi {
                window.clear();
                for k in 0..d.window {
                    let row = (i + k) % d.t;
                    for col in &cols {
                        window.push(col[row]);
                    }
                }
                periodic_i.clear();
                periodic_i.extend(per.iter().map(|p| p[i]));
                out.push(compose_base_planned(
                    air,
                    &plan,
                    x,
                    z_h_inv,
                    &window,
                    &periodic_i,
                    coeffs,
                    &mut den,
                    &mut prefix,
                ));
                x = x * d.sub;
            }
            out
        });
        for (b, part) in parts.into_iter().enumerate() {
            for (k, v) in part.into_iter().enumerate() {
                comp_d[c + d.blowup * (b * BLOCK + k)] = v;
            }
        }
    }
    comp_d
}
