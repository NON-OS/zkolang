// NONOS Operating System (AGPL-3.0-or-later)
//! One forgery per binding family in the recursive verifier. The assembly binds
//! through six families and only three had a reject gate, so a fused or
//! reordered permutation could drop fold, index or periodic and every existing
//! test would still pass.
//!
//! Each case runs on a two-query cap, which builds the identical per-query
//! machinery over a trace small enough to check without FRI. The full-coverage
//! gates prove the same bindings over 32 queries and cost an hour and gigabytes;
//! these cost a minute and megabytes, so they run on every change.

mod deep;
mod fold;
mod honest;
mod index;
mod periodic;
mod roots;
mod statement;
