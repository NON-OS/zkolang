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
mod compiler;
mod const_table;
mod count_inputs;
mod expr;
mod stmt;

use alloc::vec::Vec;

use compiler::Compiler;

use super::parse::Ast;
use super::CompileError;
use crate::isa::Op;

/// Lower an AST into a VM program ending in `Halt`.
pub fn compile(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    // Count the public inputs first, through any loops, so secret inputs index after.
    let n_public = count_inputs::count_inputs(&ast.stmts).min(u16::MAX as u64) as u16;
    let mut c = Compiler::new(ast.consts.clone(), ast.fns.clone(), n_public);
    for s in &ast.stmts {
        c.stmt(s)?;
    }
    Ok(c.finish())
}
