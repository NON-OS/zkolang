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

//! The compose strip: the inner transition's recompute as rows of witnessed
//! products under periodic schedules, replacing one constraint of the
//! inner's degree with many of degree four. `plan` holds the shape, `trace`
//! places the witness, `air` checks it; the plan is emitted by host tooling
//! from a recording of the inner's own code.

mod air;
mod plan;
mod trace;

pub use air::ComposeStrip;
pub use plan::{OpSched, OutStatement, RowSched, StripPlan, EMPTY};
