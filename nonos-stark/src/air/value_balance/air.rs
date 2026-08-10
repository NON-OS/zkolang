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

use super::super::super::field::{Felt, Fp};
use super::leg::Leg;
use alloc::vec;
use alloc::vec::Vec;

/// The scale between a note's two value limbs.
pub const LIMB_SHIFT: u64 = 1u64 << 32;

/// A note commits its value as two limbs and a copy constraint moves a cell
/// rather than scaling one, so recomposition and conservation ride one
/// constraint. That keeps the limbs as raw cells a caller can bind against.
pub struct ValueBalance {
    pub log_t: u32,
    pub legs: Vec<Leg>,
}

impl ValueBalance {
    pub(super) fn transition_impl<F: Felt>(&self, window: &[F], periodic: &[F]) -> Vec<F> {
        let (acc, lo, hi) = (window[0], window[1], window[2]);
        let value = lo + hi * F::from_base(Fp::from_u64(LIMB_SHIFT));
        vec![window[3] - acc - periodic[0] * value]
    }

    pub(super) fn signs(&self) -> Vec<Fp> {
        let n = 1usize << self.log_t;
        let mut s = vec![Fp::ZERO; n];
        for (r, v) in s.iter_mut().enumerate() {
            *v = self.legs.get(r).copied().unwrap_or(Leg::Pad).sign();
        }
        s
    }
}
