/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The compiler state and its register allocator, one concern per file. The
//! allocator reuses freed registers, and a binding resolves newest-first with an
//! alias-aware reclaim on shadowing, which keeps register pressure at expression
//! depth. The two bounds guard against a runaway unroll or a recursive inline.

mod alloc;
mod bind_fresh;
mod finish;
mod free_dead;
mod free_reg;
mod hide;
mod io_index;
mod limits;
mod lookup;
mod loop_const;
mod new;
mod rebind;
mod reg_in_use;
mod release;
mod state;
mod take_scalar;
mod val;
mod work;

pub(crate) use limits::{MAX_EXPR_DEPTH, MAX_INLINE, MAX_UNROLL};
pub(crate) use state::Compiler;
pub(crate) use val::Val;
