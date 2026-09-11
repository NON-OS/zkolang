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

//! The public-input region: one committed word per row, each pinned by a boundary. It is the
//! surface where a circuit's public statement enters the trace, so an assembly copy-constrains
//! each computed word to its row here and the verifier reads the statement off the boundaries.
//! The binding is positive: a word is tied to the cell that computed it, not merely asserted
//! equal to a constant a prover could also satisfy elsewhere.

mod air;

pub use air::Publics;
