/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2026 compiler. The stages, in pipeline order, are documented in
 * `docs/compiler-architecture.md`.
 */

pub mod diag;
pub mod interp;
pub mod sema;
pub mod source;
pub mod syntax;
pub mod tir;
