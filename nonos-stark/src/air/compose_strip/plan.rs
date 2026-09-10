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

//! The strip's shape as data, so the region and the host emitter share one source. A plan is
//! the rows, each row its product lanes and their two operand schedules, the echo cells that
//! carry values in from other rows by copy cycle, and the accumulator coefficients. A dense
//! operand schedule is a coefficient per candidate cell plus a constant, zero everywhere it
//! does not read, so the folded scalar edges of the tape cost nothing in the trace. The plan
//! is emitted once from a recording of the inner's arithmetic and consumed unchanged by the
//! region, which is why the two cannot disagree about the layout.

use super::super::super::field::Fp;
use alloc::vec::Vec;

/// One operand as a dense schedule: a coefficient per candidate cell plus a
/// constant, zero everywhere it does not read. Candidates are the previous
/// row's products, this row's products, then this row's echo bank.
#[derive(Clone)]
pub struct OpSched {
    pub coeffs: Vec<Fp>,
    pub constant: Fp,
}

/// One strip row: the value index each lane and echo cell carries (indices
/// into a caller-side value table; `EMPTY` for an unused lane), the two
/// operand schedules per lane, and per output the coefficient each lane's
/// product contributes to its accumulator.
#[derive(Clone)]
pub struct RowSched {
    pub lanes: Vec<u32>,
    pub echoes: Vec<u32>,
    pub ops: Vec<(OpSched, OpSched)>,
    pub acc: Vec<Vec<Fp>>,
}

pub const EMPTY: u32 = u32::MAX;

/// The statement side of one strip output: a coefficient per base input
/// lane plus the folded constant. The flat compose region evaluates this
/// over its own frame and periodic cells in its pin constraint, so the
/// strip carries products only.
#[derive(Clone)]
pub struct OutStatement {
    pub input_coeffs: Vec<(u32, Fp)>,
    pub constant: Fp,
}

/// The strip's whole shape: row 0 is the inert init row (accumulators pinned
/// zero, no lanes), rows 1..=n carry the schedule. A pure function of the
/// inner's constraint code by way of the tape, never typed.
#[derive(Clone)]
pub struct StripPlan {
    pub k: usize,
    pub echo_width: usize,
    pub n_out: usize,
    pub rows: Vec<RowSched>,
}

impl StripPlan {
    /// Candidate cells an operand schedule reads.
    pub fn candidates(&self) -> usize {
        2 * self.k + self.echo_width
    }
}
