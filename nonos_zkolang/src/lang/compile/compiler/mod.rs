/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The compiler state and its register allocator, one concern per file. The
//! allocator reuses freed registers, and a binding resolves newest-first with an
//! alias-aware reclaim on shadowing, which keeps register pressure at expression
//! depth. The two bounds guard against a runaway unroll or a recursive inline.

mod alloc;
mod finish;
mod lookup;
mod loop_const;
mod new;
mod rebind;
mod reg_in_use;
mod release;
mod state;
mod take_scalar;
mod val;

pub(crate) use state::Compiler;
pub(crate) use val::Val;

/// The largest number of iterations a single loop may unroll to, a fail-fast guard
/// before the trace-length cap catches anything larger at prove time.
pub(crate) const MAX_UNROLL: u64 = 65_536;

/// The deepest a chain of inlined calls may nest, which turns a recursive call into
/// a compile error rather than a non-terminating inline.
pub(crate) const MAX_INLINE: usize = 256;
