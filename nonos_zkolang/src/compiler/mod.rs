/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2026 compiler. The stages, in pipeline order, are documented in
 * `docs/compiler-architecture.md`.
 */

pub mod codegen;
pub mod diag;
pub mod driver;
pub mod gadget;
pub mod interp;
pub mod lower;
pub mod opt;
pub mod package;
pub mod schedule;
pub mod sema;
pub mod source;
pub mod ssa;
pub mod std_crate;
pub mod syntax;
pub mod tir;
