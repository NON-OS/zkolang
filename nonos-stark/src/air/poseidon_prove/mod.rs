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

//! The poseidon-transcript provers, streamed and pruned like everything else.
//! `trace` commits the columns, `queries` opens them, `sidecar` opens the
//! committed periodic rows; `pre` sequences the preprocessed protocol. What a
//! pass computes lives in one place; a prover is a transcript order.

mod pre;
mod queries;
mod sidecar;
mod trace;

pub use pre::stark_prove_poseidon_pre_pub;
