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

//! The strip's witness. Each row places its product lanes, fills its echo cells from the value
//! table the plan's indices point into, and steps the accumulators forward, so the totals stand
//! at the final row. The plan rows come first and inert padding follows; the padding holds the
//! accumulator totals rather than resetting them, so the final binding may read any padded row.
//! This is the single source of the layout the region checks, so a caller proves against
//! exactly the assignment the constraints expect.

use super::plan::{StripPlan, EMPTY};
use super::super::super::field::Fp;
use alloc::vec::Vec;

/// A row holds the product lanes, the echo bank, then the accumulators.
pub(super) fn width(p: &StripPlan) -> usize {
    p.k + p.echo_width + p.n_out
}

/// The witness over `rows` total rows: plan rows first, inert padding after.
/// `vals` is the value table the plan's lane and echo indices point into.
/// Accumulators run forward and hold their totals through the padding, so
/// the final binding may read any padded row.
pub(super) fn build(p: &StripPlan, vals: &[Fp], rows: usize) -> Vec<Fp> {
    let w = width(p);
    let mut tr = alloc::vec![Fp::ZERO; rows * w];
    let mut acc = alloc::vec![Fp::ZERO; p.n_out];
    for (r, row) in p.rows.iter().enumerate() {
        for (l, id) in row.lanes.iter().enumerate() {
            if *id != EMPTY {
                tr[r * w + l] = vals[*id as usize];
            }
        }
        for (e, id) in row.echoes.iter().enumerate() {
            tr[r * w + p.k + e] = vals[*id as usize];
        }
        for j in 0..p.n_out {
            for (l, c) in row.acc[j].iter().enumerate() {
                if row.lanes[l] != EMPTY {
                    acc[j] = acc[j] + *c * vals[row.lanes[l] as usize];
                }
            }
            tr[r * w + p.k + p.echo_width + j] = acc[j];
        }
    }
    for r in p.rows.len()..rows {
        for j in 0..p.n_out {
            tr[r * w + p.k + p.echo_width + j] = acc[j];
        }
    }
    tr
}
