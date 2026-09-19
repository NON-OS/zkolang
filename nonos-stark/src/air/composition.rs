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

//! Combining an AIR's constraints into one composition value at a coset point.
//! Each transition constraint becomes a quotient by the trace-domain vanishing
//! polynomial (with the exempt final rows multiplied back in), each boundary a
//! quotient by its single vanishing point, and all are folded under transcript
//! coefficients. Prover and verifier both call this, so the algebra is identical
//! on the two sides by construction.

use super::super::field::{Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// Total number of random coefficients an AIR's composition consumes.
pub(super) fn num_coeffs<A: Air>(air: &A) -> usize {
    air.num_transition() + air.boundary().len()
}

/// The evaluation-domain sizing derived from the AIR: `(log_n, fri_log_blowup)`.
///
/// The composition degree is below `constraint_degree * trace_len`, so the FRI
/// degree bound `B` is that rounded up to a power of two, and the evaluation
/// domain is `2 * B` (a rate of one half for the low-degree test). Both prover
/// and verifier call this, so they agree on the domain without passing sizes.
pub(super) fn domain_params<A: Air>(air: &A) -> (u32, u32) {
    domain_params_blown(air, 0)
}

/// The evaluation domain and FRI blowup with `extra_blowup_bits` of low-degree
/// headroom folded into the FRI rate. Zero reproduces the minimal 2 * B domain
/// (rate one half). Each added bit doubles the domain and halves the rate, so a
/// single FRI query catches a non-low-degree codeword with proportionally higher
/// probability: this is the query-count-versus-soundness lever a deployment vector
/// sets to reach 128-bit security without paying for a hundred queries. The extra
/// cost is a larger LDE, all prover side.
pub(super) fn domain_params_blown<A: Air>(air: &A, extra_blowup_bits: u32) -> (u32, u32) {
    let t = 1usize << air.log_trace_len();
    let degree = air.constraint_degree().max(1);
    let bound = (degree * t).next_power_of_two();
    let n = (2 * bound) << extra_blowup_bits;
    let log_n = n.trailing_zeros();
    let fri_log_blowup = log_n - bound.trailing_zeros();
    (log_n, fri_log_blowup)
}

/// The composition value at coset point `x`, given the trace window
/// `[f(x), f(g*x), ...]`, the periodic columns evaluated at `x`, and the
/// transcript coefficients. `g` is the trace-domain generator. `x` lies off the
/// trace domain, so every divisor is invertible.
pub fn compose<A: Air>(air: &A, g: Fp, x: Fp, window: &[Fp], periodic: &[Fp], coeffs: &[Fp]) -> Fp {
    let t = 1u64 << air.log_trace_len();
    let z_h_inv = (x.pow(t) - Fp::ONE).inv();

    // The final `window_size - 1` rows have no successor, so exempt them by
    // multiplying the transition numerator by their vanishing points.
    let mut exempt = Fp::ONE;
    for k in 1..air.window_size() {
        exempt = exempt * (x - g.pow(t - k as u64));
    }

    let mut acc = Fp::ZERO;
    let transition = air.transition(window, periodic);
    for (value, coeff) in transition.iter().zip(coeffs.iter()) {
        acc = acc + *coeff * (*value * exempt * z_h_inv);
    }

    // Boundary quotients read column `col` at window offset zero, `window[col]`.
    let boundary_coeffs = &coeffs[transition.len()..];
    for ((col, row, expected), coeff) in air.boundary().iter().zip(boundary_coeffs.iter()) {
        let quotient = (window[*col] - *expected) * (x - g.pow(*row as u64)).inv();
        acc = acc + *coeff * quotient;
    }

    acc
}

/// The composition value at an extension point `z in Fp2`, the out-of-domain
/// evaluation a money-grade STARK samples. Same algebra as `compose`, over the
/// extension: base-field constants (the trace generator powers, boundary values)
/// enter through `Fp2::from_base`, and the batching coefficients are drawn from the
/// extension for the same soundness. On a base-embedded point and base-embedded
/// inputs it agrees with `compose` embedded, by construction.
/// The parts of the composition that depend on the AIR and the domain but not
/// on the point: the exemption points, the boundary list itself, and the trace
/// domain point each boundary sits on. Built once per proof. Evaluating them
/// inside the per point loop cost one exponentiation per boundary per row and
/// rebuilt the boundary list every row, which on the deployed outer is 1,186 of
/// each for every one of millions of points.
pub struct ComposePlan {
    t: u64,
    exempt_pts: Vec<Fp>,
    boundary: Vec<(usize, usize, Fp)>,
    boundary_pts: Vec<Fp>,
    /*
     * The boundary list grouped by the row it pins. On the deployed outer
     * 1,190 boundaries sit on 228 distinct rows, two of which carry 510 of
     * them, so a denominator per boundary inverts the same value five times
     * over on average. One point per distinct row, in first-seen order, and
     * the boundary indices that share it.
     */
    row_pts: Vec<Fp>,
    row_members: Vec<Vec<usize>>,
}

impl ComposePlan {
    pub fn new<A: Air>(air: &A, g: Fp) -> ComposePlan {
        let t = 1u64 << air.log_trace_len();
        let exempt_pts = (1..air.window_size())
            .map(|k| g.pow(t - k as u64))
            .collect();
        let boundary = air.boundary();
        let boundary_pts: Vec<Fp> = boundary
            .iter()
            .map(|(_, row, _)| g.pow(*row as u64))
            .collect();

        let mut slot_of_row: alloc::collections::BTreeMap<usize, usize> =
            alloc::collections::BTreeMap::new();
        let mut row_pts: Vec<Fp> = Vec::new();
        let mut row_members: Vec<Vec<usize>> = Vec::new();
        for (j, ((_, row, _), pt)) in boundary.iter().zip(boundary_pts.iter()).enumerate() {
            let slot = *slot_of_row.entry(*row).or_insert_with(|| {
                row_pts.push(*pt);
                row_members.push(Vec::new());
                row_pts.len() - 1
            });
            row_members[slot].push(j);
        }
        ComposePlan {
            t,
            exempt_pts,
            boundary,
            boundary_pts,
            row_pts,
            row_members,
        }
    }

    /// `1 / (x^t - 1)` for a base field point. On a coset of the evaluation
    /// domain `x^t` is the same for every point of the coset, because the
    /// subgroup the cosets share has order `t`, so a prover walking a coset
    /// takes this once per coset rather than once per point.
    pub fn vanishing_inv(&self, x: Fp) -> Fp {
        (x.pow(self.t) - Fp::ONE).inv()
    }
}

/// The composition at a base field point, which is every point of the
/// evaluation domain. Same value `compose_ext_planned` returns at that point
/// lifted to the extension, computed without the lift: the transition bodies
/// run in the base field, each product is one base multiplication and one
/// scaling of a coefficient, and the boundary denominators are one per
/// distinct row rather than one per boundary. The composition polynomial the
/// prover commits is therefore unchanged, and so is every byte after it.
///
/// `z_h_inv` is the coset's vanishing inverse from `vanishing_inv`, which is
/// what makes the per point cost free of exponentiation.
#[allow(clippy::too_many_arguments)]
pub fn compose_base_planned<A: Air>(
    air: &A,
    plan: &ComposePlan,
    x: Fp,
    z_h_inv: Fp,
    window: &[Fp],
    periodic: &[Fp],
    coeffs: &[Fp2],
    den: &mut Vec<Fp>,
    prefix: &mut Vec<Fp>,
) -> Fp2 {
    let mut exempt = Fp::ONE;
    for p in &plan.exempt_pts {
        exempt = exempt * (x - *p);
    }
    let scale = exempt * z_h_inv;

    let mut acc = Fp2::ZERO;
    let transition = air.transition(window, periodic);
    for (value, coeff) in transition.iter().zip(coeffs.iter()) {
        acc = acc + coeff.mul_base(*value * scale);
    }

    den.clear();
    den.extend(plan.row_pts.iter().map(|p| x - *p));
    crate::poly::batch_inv_in_place(den, prefix);

    let boundary_coeffs = &coeffs[transition.len()..];
    for (inv, members) in den.iter().zip(plan.row_members.iter()) {
        for &j in members {
            let (col, _, expected) = plan.boundary[j];
            acc = acc + boundary_coeffs[j].mul_base((window[col] - expected) * *inv);
        }
    }

    acc
}

/// The composition at `z` against a prepared plan, with the caller owning the
/// scratch the boundary denominators are inverted in. One inversion serves the
/// whole boundary set: at 1,186 boundaries that is one exponentiation by p-2
/// instead of 1,186 of them.
pub fn compose_ext_planned<A: AirExt>(
    air: &A,
    plan: &ComposePlan,
    z: Fp2,
    window: &[Fp2],
    periodic: &[Fp2],
    coeffs: &[Fp2],
    den: &mut Vec<Fp2>,
    prefix: &mut Vec<Fp2>,
) -> Fp2 {
    let z_h_inv = (z.pow(plan.t) - Fp2::ONE).inv();

    let mut exempt = Fp2::ONE;
    for p in &plan.exempt_pts {
        exempt = exempt * (z - Fp2::from_base(*p));
    }

    let mut acc = Fp2::ZERO;
    let transition = air.transition_ext(window, periodic);
    for (value, coeff) in transition.iter().zip(coeffs.iter()) {
        acc = acc + *coeff * (*value * exempt * z_h_inv);
    }

    den.clear();
    den.extend(plan.boundary_pts.iter().map(|p| z - Fp2::from_base(*p)));
    crate::poly::batch_inv_in_place(den, prefix);

    let boundary_coeffs = &coeffs[transition.len()..];
    for (((col, _, expected), coeff), inv) in plan
        .boundary
        .iter()
        .zip(boundary_coeffs.iter())
        .zip(den.iter())
    {
        acc = acc + *coeff * ((window[*col] - Fp2::from_base(*expected)) * *inv);
    }

    acc
}

pub fn compose_ext<A: AirExt>(
    air: &A,
    g: Fp,
    z: Fp2,
    window: &[Fp2],
    periodic: &[Fp2],
    coeffs: &[Fp2],
) -> Fp2 {
    let plan = ComposePlan::new(air, g);
    let (mut den, mut prefix) = (Vec::new(), Vec::new());
    compose_ext_planned(
        air,
        &plan,
        z,
        window,
        periodic,
        coeffs,
        &mut den,
        &mut prefix,
    )
}

#[cfg(test)]
mod tests {
    use super::super::index_scalar::IndexScalar;
    use super::super::spec::{Air, AirExt};
    use super::*;
    use crate::fri::root_of_unity;

    /// The base field kernel is the extension kernel at a base point, value
    /// for value. Checked on an AIR with periodic columns and a boundary,
    /// over several cosets, with a boundary list padded so two boundaries
    /// share a row and the grouping has something to group.
    #[test]
    fn the_base_kernel_equals_the_extension_kernel_at_base_points() {
        let air = IndexScalar::new(6, 5);
        let log_t = air.log_trace_len();
        let t = 1u64 << log_t;
        let g = root_of_unity(log_t);
        let plan = ComposePlan::new(&air, g);
        let width = air.trace_width();
        let window_len = width * air.window_size();
        let n_periodic = air.periodic_columns().len();
        let n_coeffs = air.num_transition() + air.boundary().len();

        let coeffs: Vec<Fp2> = (0..n_coeffs as u64)
            .map(|i| Fp2 {
                c0: Fp::from_u64(3 * i + 7),
                c1: Fp::from_u64(11 * i + 1),
            })
            .collect();
        let window: Vec<Fp> = (0..window_len as u64)
            .map(|i| Fp::from_u64(5 * i + 2))
            .collect();
        let periodic: Vec<Fp> = (0..n_periodic as u64)
            .map(|i| Fp::from_u64(13 * i + 3))
            .collect();
        let window_e: Vec<Fp2> = window.iter().map(|v| Fp2::from_base(*v)).collect();
        let periodic_e: Vec<Fp2> = periodic.iter().map(|v| Fp2::from_base(*v)).collect();

        let (mut den, mut prefix) = (Vec::new(), Vec::new());
        let (mut den_e, mut prefix_e) = (Vec::new(), Vec::new());
        /*
         * Points off the trace subgroup, the way domain points are: a shift
         * times a power of a root of unity of a larger order.
         */
        let omega = root_of_unity(log_t + 3);
        for shift in [7u64, 9, 100] {
            let shift = Fp::from_u64(shift);
            let z_h_inv = plan.vanishing_inv(shift);
            for i in [0u64, 1, 5, 8, 27] {
                let x = shift * omega.pow(i * 8);
                assert_eq!(
                    z_h_inv,
                    (x.pow(t) - Fp::ONE).inv(),
                    "x^t is constant on a coset"
                );
                let base = compose_base_planned(
                    &air,
                    &plan,
                    x,
                    z_h_inv,
                    &window,
                    &periodic,
                    &coeffs,
                    &mut den,
                    &mut prefix,
                );
                let ext = compose_ext_planned(
                    &air,
                    &plan,
                    Fp2::from_base(x),
                    &window_e,
                    &periodic_e,
                    &coeffs,
                    &mut den_e,
                    &mut prefix_e,
                );
                assert_eq!(
                    base, ext,
                    "the kernels disagree at shift {shift:?} index {i}"
                );
            }
        }
    }

    /// The row grouping covers every boundary exactly once, in a plan built
    /// over a boundary list where rows repeat.
    #[test]
    fn the_row_grouping_is_a_partition_of_the_boundary_list() {
        let air = IndexScalar::new(6, 5);
        let plan = ComposePlan::new(&air, root_of_unity(air.log_trace_len()));
        let mut seen = alloc::vec![0usize; plan.boundary.len()];
        for (slot, members) in plan.row_members.iter().enumerate() {
            for &j in members {
                seen[j] += 1;
                assert_eq!(
                    plan.boundary_pts[j], plan.row_pts[slot],
                    "member {j} is on another row"
                );
            }
        }
        assert!(
            seen.iter().all(|&n| n == 1),
            "every boundary belongs to exactly one row"
        );
        assert_eq!(plan.row_pts.len(), plan.row_members.len());
    }
}
