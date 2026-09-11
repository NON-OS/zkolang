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

//! The compose strip as an AIR. The recursion recomputes the inner transition, and doing it as
//! one constraint carries the inner's degree, thirteen, into the outer and sets its blowup.
//! The strip trades that one high-degree constraint for many low-degree ones: each row holds a
//! few product lanes, each lane checks `out = opA * opB` where the operands are
//! schedule-weighted sums over the row's candidate cells, and an accumulator folds the products
//! into the transition values the composition consumes. The per-row combination coefficients
//! ride periodic columns, so one uniform transition covers every row and the layout stays a
//! pure function of the inner's own code. Row zero is an inert init row with the accumulators
//! pinned to zero; padded rows read zero schedules, which pins their products to zero and holds
//! the accumulator totals for the final binding. The constraint degree is four, schedule times
//! cell per operand, squared by the product.

use super::plan::{StripPlan, EMPTY};
use super::trace;
use super::super::super::field::{Felt, Fp, Fp2};
use super::super::spec::{Air, AirExt};
use alloc::vec::Vec;

/// The strip region: one uniform product check per lane and one step per
/// output accumulator, the per-row combinations riding periodic schedules.
/// Row 0 is the inert init row, accumulators pinned zero; products fill rows
/// 1.. and the totals hold through the padding for the final binding.
pub struct ComposeStrip {
    plan: StripPlan,
    log_rows: u32,
}

impl ComposeStrip {
    pub fn new(plan: StripPlan) -> ComposeStrip {
        let mut log_rows = 1u32;
        while (1usize << log_rows) < plan.rows.len() {
            log_rows += 1;
        }
        ComposeStrip { plan, log_rows }
    }

    /// The witness from a value table the plan's indices point into.
    pub fn trace(&self, vals: &[Fp]) -> Vec<Fp> {
        trace::build(&self.plan, vals, 1 << self.log_rows)
    }

    /// The accumulator cell for output `j`, for the final binding.
    pub fn acc_col(&self, j: usize) -> usize {
        self.plan.k + self.plan.echo_width + j
    }

    /// The echo cell columns of row `r`, with the value index each carries:
    /// the cycle endpoints the assembly binds to the values' true homes.
    pub fn echo_cells(&self, r: usize) -> Vec<(usize, u32)> {
        self.plan.rows[r]
            .echoes
            .iter()
            .enumerate()
            .map(|(e, id)| (self.plan.k + e, *id))
            .collect()
    }

    pub fn used_rows(&self) -> usize {
        self.plan.rows.len()
    }

    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let (k, e, n_out) = (self.plan.k, self.plan.echo_width, self.plan.n_out);
        let w = trace::width(&self.plan);
        let n_cand = self.plan.candidates();
        let stride = n_cand + 1;
        let mut res = Vec::with_capacity(k + n_out);

        /*
         * The candidate bank a schedule may read, in one fixed order so a coefficient index
         * means the same cell on every row: first this row's k previous-row products (window
         * positions 0..k), then this row's own k products (at w + 0..k in the next-row half of
         * the two-row window), then this row's e echo cells. A schedule is dense over this
         * bank, and an operand is its coefficient-weighted sum plus a constant.
         */
        let bank = |i: usize| -> F {
            if i < k {
                window[i]
            } else if i < 2 * k {
                window[w + (i - k)]
            } else {
                window[w + k + (i - 2 * k)]
            }
        };
        /*
         * One product check per lane. The two operand schedules for lane l occupy a block of
         * 2 * stride periodic columns; within each the last entry is the constant and the rest
         * are the bank coefficients. Assembling both operands and forcing the witnessed product
         * cell to equal their product is the whole per-lane constraint, degree four: a schedule
         * times a bank cell is degree two, and the two operands multiply.
         */
        for l in 0..k {
            let base = 2 * stride * l;
            let mut a = periodic[base + n_cand];
            let mut b = periodic[base + stride + n_cand];
            for i in 0..n_cand {
                a = a + periodic[base + i] * bank(i);
                b = b + periodic[base + stride + i] * bank(i);
            }
            res.push(window[w + l] - a * b);
        }
        /*
         * The accumulators fold the products into the transition values the composition
         * consumes. Each output's next value is its current value plus a schedule-weighted sum
         * of this row's products, so across the strip an output accumulates exactly its share
         * of every product. The init row pins these to zero and the final row holds the totals,
         * which the assembly binds to the composition's out cells by cycle.
         */
        let acc0 = 2 * stride * k;
        for j in 0..n_out {
            let mut step = window[w + k + e + j] - window[k + e + j];
            for l in 0..k {
                step = step - periodic[acc0 + j * k + l] * window[w + l];
            }
            res.push(step);
        }
        res
    }
}

impl Air for ComposeStrip {
    fn log_trace_len(&self) -> u32 {
        self.log_rows
    }

    fn trace_width(&self) -> usize {
        trace::width(&self.plan)
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        // Schedule times cell per operand, two operands multiplied.
        4
    }

    fn num_transition(&self) -> usize {
        self.plan.k + self.plan.n_out
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        // The schedule at anchor r is the consumer row r + 1's, so every
        // column shifts down one; anchors past the plan read zeros, which
        // pins padded products to zero and holds the accumulators.
        let rows = 1usize << self.log_rows;
        let n_cand = self.plan.candidates();
        let stride = n_cand + 1;
        let n_cols = 2 * stride * self.plan.k + self.plan.n_out * self.plan.k;
        let mut cols = alloc::vec![alloc::vec![Fp::ZERO; rows]; n_cols];
        for (r, row) in self.plan.rows.iter().enumerate().skip(1) {
            let a = r - 1;
            for l in 0..self.plan.k {
                if row.lanes[l] == EMPTY {
                    continue;
                }
                let (oa, ob) = &row.ops[l];
                let base = 2 * stride * l;
                for (i, c) in oa.coeffs.iter().enumerate() {
                    cols[base + i][a] = *c;
                }
                cols[base + n_cand][a] = oa.constant;
                for (i, c) in ob.coeffs.iter().enumerate() {
                    cols[base + stride + i][a] = *c;
                }
                cols[base + stride + n_cand][a] = ob.constant;
            }
            let acc0 = 2 * stride * self.plan.k;
            for j in 0..self.plan.n_out {
                for l in 0..self.plan.k {
                    cols[acc0 + j * self.plan.k + l][a] = row.acc[j][l];
                }
            }
        }
        cols
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // The init row: every accumulator starts at zero, and its lanes are
        // empty by construction of the plan.
        (0..self.plan.n_out).map(|j| (self.acc_col(j), 0, Fp::ZERO)).collect()
    }
}

impl AirExt for ComposeStrip {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
