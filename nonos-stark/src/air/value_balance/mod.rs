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

//! Value conservation for a shielded transfer, as a small AIR. The region proves that the
//! signed sum of the note values on its rows is zero, so inputs equal outputs plus fee, with
//! no amount revealed. It is the arithmetic counterpart of the Lean no-inflation theorem: the
//! Lean side proves conservation survives the field once the amounts are range-bounded, and
//! this side is the circuit that enforces the sum. `air` holds the region and its transition,
//! `spec` its `Air` shape, `leg` the public input-or-output sign, `trace` the witness.

mod air;
mod spec;
mod leg;
mod trace;

pub use air::{ValueBalance, LIMB_SHIFT};
pub use leg::Leg;
