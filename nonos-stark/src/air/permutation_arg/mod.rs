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

//! The permutation argument, the single grand product that enforces every copy binding at
//! once. `cycles` turns binding classes into a permutation, `arg` runs the challenged running
//! product whose unit boundary is the whole set of bindings, `disjoint` proves the classes do
//! not collide so one shared permutation is safe, and `layable` checks the classes an assembly
//! hands over can be laid down. The Lean `Wiring` and `GrandProduct` modules prove the two
//! properties this rests on: disjointness preserves earlier bindings, and the accumulator
//! computes the product its boundary claims.

mod arg;
mod cycles;
mod disjoint;
mod layable;

pub use cycles::{Cell, WirePermutation};
pub use arg::WiredPermutationArg;
pub use disjoint::classes_are_disjoint;
pub use layable::classes_are_layable;
