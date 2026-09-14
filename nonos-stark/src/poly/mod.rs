// NONOS Operating System (AGPL-3.0-or-later)
//! Polynomials over the field: evaluation in coefficient form, Lagrange
//! evaluation of the low-degree extension, and the number-theoretic transform
//! with the coset low-degree extension built on it.

mod blind;
mod eval;
mod inv;
mod lagrange;
mod lde;
mod ntt;

pub use blind::blind_coeffs;
pub use eval::{eval, eval_ext};
pub use inv::{batch_inv, batch_inv_in_place};
pub use lagrange::{eval_cols_on_subgroup, eval_cols_on_subgroup_ext, eval_lagrange, eval_lagrange_ext};
pub use lde::{lde, lde_from_coeffs};
pub use ntt::{intt, ntt};
