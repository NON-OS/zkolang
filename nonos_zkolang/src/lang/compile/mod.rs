/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Lowering: the AST to a flat VM program. The language is single-assignment at the
//! source level, but the compiler reuses physical registers, which keeps register
//! pressure at expression depth. Register indices stay compile-time constants, all
//! the step AIR's register binding needs, so reuse is invisible to it. The work is
//! split by concern: the compiler state and allocator, the statement lowering, the
//! expression lowering, the constant tables, and the arrays.

mod array;
mod compiled;
mod compiler;
mod const_table;
mod count_inputs;
mod count_secrets;
mod expr;
mod stmt;

use alloc::vec::Vec;

use compiler::Compiler;

pub use compiled::Compiled;

use super::parse::Ast;
use super::CompileError;
use crate::isa::Op;

// Lower an AST as given, without optimizing. The public inputs and secrets are counted
// first, through any loops, so secrets index after the public prefix and comparison advice
// indexes after the secrets.
fn lower(ast: &Ast) -> Result<Compiled, CompileError> {
    let n_public = count_inputs::count_inputs(&ast.stmts).min(u16::MAX as u64) as u16;
    let n_secret = count_secrets::count_secrets(&ast.stmts).min(u16::MAX as u64) as u16;
    let mut c = Compiler::new(ast.consts.clone(), ast.fns.clone(), n_public, n_secret);
    for s in &ast.stmts {
        c.stmt(s)?;
    }
    Ok(c.finish())
}

/// Lower an AST into a VM program with its advice plan, optimizing first so the trace is
/// smaller while the proof is unchanged.
pub fn compile_full(ast: &Ast) -> Result<Compiled, CompileError> {
    lower(&super::optimize::optimize(ast))
}

/// Lower an AST into a VM program ending in `Halt`.
pub fn compile(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    compile_full(ast).map(|c| c.ops)
}

/// Lower an AST into a VM program without the optimizer, for checking that optimization
/// preserves behavior.
pub fn compile_unoptimized(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    lower(ast).map(|c| c.ops)
}
