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

//! The preprocessed-periodic prover, streamed like `prove_ext` and sharing its
//! passes. What differs is the sidecar: the periodic tree is committed through
//! the registration helper, the periodic values at z ride the proof, and DEEP
//! carries one quotient per periodic column so the verifier can hold the
//! periodic root as a baked constant.

mod deep;
mod queries;
mod rounds;
mod run;

pub(in crate::air) use deep::over_domain as pre_deep_over_domain;
pub use rounds::stark_prove_ext_rounds;
pub use run::{
    stark_prove_ext_preprocessed, stark_prove_ext_preprocessed_tree,
    stark_prove_ext_preprocessed_watched,
};
