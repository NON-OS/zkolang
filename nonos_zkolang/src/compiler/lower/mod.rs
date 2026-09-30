/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Lowering a checked program to SSA: every call inlined, every loop unrolled, every branch
 * guarded (section 21.4), every value scalarized into its slots (section 6), and every
 * failure condition of section 14.1 a constraint conditioned on the guard.
 */

mod assign;
mod bind;
mod binop;
mod binop_scalar;
mod bits_conv;
mod block;
mod builtin;
mod call;
mod cast;
mod chain;
mod compound;
mod conv;
mod cx;
mod entry;
mod error;
mod exits;
mod expr;
mod guard;
mod if_;
mod inline;
mod inputs;
mod inputs_enum;
mod int;
mod int64;
mod int_bits;
mod int_bitwise;
mod int_cmp;
mod int_div;
mod iteration;
pub(crate) mod layout;
mod lit;
mod locals;
mod lockstep;
mod loop_const;
mod loop_for;
mod loops;
mod match_;
mod pat_cond;
mod pat_parts;
mod pat_range;
mod place;
mod place_access;
mod place_index;
mod place_write;
mod pow;
mod power;
mod shift;
mod unary;
mod value_slots;
mod variant;
mod wrapping;

pub use entry::{lower_function, lower_program};
pub use error::LowerError;
