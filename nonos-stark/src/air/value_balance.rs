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

//! Value conservation for a join-split, over values that are still split into
//! the limbs a note commits to.
//!
//! A shielded note commits its value as two limbs, the low thirty-two bits and
//! the rest, because that is how `ShieldedPool._computeCommitment` packs it. A
//! plain running-sum accumulator would need the recomposed value, and no copy
//! constraint can express `v = lo + 2^32 * hi`: a copy constraint moves a cell,
//! it cannot scale one. Recomposing in a separate region and binding the result
//! would work but adds a region and two more bindings per term, and every extra
//! binding is another chance to pin the wrong cell.
//!
//! So recomposition and conservation are one constraint here:
//!
//! ```text
//!   acc[i+1] = acc[i] + s[i] * (lo[i] + 2^32 * hi[i])
//! ```
//!
//! with the running total pinned to zero at both ends, so the signed terms
//! cancel. `s` is a periodic column carrying +1 for an input, -1 for an output
//! or a public leg, and 0 for padding. The layout of a batch is public
//! structure, not witness, so the signs belong in a periodic column: a prover
//! cannot choose which row is an input.
//!
//! The payoff is that `lo` and `hi` stay raw trace cells, so each one binds
//! directly to the corresponding limb inside its note commitment. Conservation
//! is then over exactly the values the notes committed to, not over numbers a
//! prover asserted alongside them.
//!
//! Degree stays one: `s` is periodic, so the product is linear in the trace.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec;
use alloc::vec::Vec;

/// The scale between a note's two value limbs.
pub const LIMB_SHIFT: u64 = 1u64 << 32;

/// The role of one row's term in the balance.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Leg {
    /// A note being spent: adds to the total.
    Input,
    /// A note being created, or value leaving publicly (public amount, fee).
    Output,
    /// Structural padding: contributes nothing.
    Pad,
}

impl Leg {
    fn sign(self) -> Fp {
        match self {
            Leg::Input => Fp::ONE,
            Leg::Output => Fp::ZERO - Fp::ONE,
            Leg::Pad => Fp::ZERO,
        }
    }
}

pub struct ValueBalance {
    pub log_t: u32,
    /// One entry per row: which side of the balance this row's value sits on.
    pub legs: Vec<Leg>,
}

impl ValueBalance {
    /// `window = [acc, lo, hi, acc', lo', hi']`, periodic `[s]`.
    fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let (acc, lo, hi) = (window[0], window[1], window[2]);
        let acc_next = window[3];
        let s = periodic[0];
        let value = lo + hi * F::from_base(Fp::from_u64(LIMB_SHIFT));
        vec![acc_next - acc - s * value]
    }

    /// The witness: the running total, and each row's value limbs. The caller
    /// supplies `(lo, hi)` per row in the same order as `legs`, so the trace and
    /// the sign column cannot drift apart.
    pub fn trace(&self, terms: &[(Fp, Fp)]) -> Vec<Fp> {
        let n = 1usize << self.log_t;
        let mut t = vec![Fp::ZERO; n * 3];
        let mut acc = Fp::ZERO;
        for r in 0..n {
            let (lo, hi) = terms.get(r).copied().unwrap_or((Fp::ZERO, Fp::ZERO));
            t[r * 3] = acc;
            t[r * 3 + 1] = lo;
            t[r * 3 + 2] = hi;
            let leg = self.legs.get(r).copied().unwrap_or(Leg::Pad);
            acc = acc + leg.sign() * (lo + hi * Fp::from_u64(LIMB_SHIFT));
        }
        t
    }
}

impl Air for ValueBalance {
    fn log_trace_len(&self) -> u32 {
        self.log_t
    }

    fn trace_width(&self) -> usize {
        3
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        1
    }

    fn num_transition(&self) -> usize {
        1
    }

    fn periodic_columns(&self) -> Vec<Vec<Fp>> {
        let n = 1usize << self.log_t;
        let mut s = vec![Fp::ZERO; n];
        for (r, v) in s.iter_mut().enumerate() {
            *v = self.legs.get(r).copied().unwrap_or(Leg::Pad).sign();
        }
        vec![s]
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Starts at zero and returns to zero: inputs equal outputs plus the
        // public legs, so the batch neither mints nor destroys value.
        let last = (1usize << self.log_t) - 1;
        vec![(0, 0, Fp::ZERO), (0, last, Fp::ZERO)]
    }
}

impl AirExt for ValueBalance {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}
