/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The compile entry points: check names, optimize or not, and lower. */

use alloc::vec::Vec;

use super::compiled::Compiled;
use super::lower::lower;
use super::name_check::check_names;
use crate::isa::Op;
use crate::lang::parse::Ast;
use crate::lang::CompileError;

/**
 * Lower an AST into a VM program with its advice plan, optimizing first. The optimizer
 * rewrites the tree before names and shapes are checked, and a fold can drop an undefined
 * name or let an array through scalar arithmetic, so the program is first lowered as
 * written. Its errors stand, except running out of registers, instructions or input
 * indices, which the optimized program may avoid; a program is valid or not whether or
 * not it is optimized.
 */
pub fn compile_full(ast: &Ast) -> Result<Compiled, CompileError> {
    check_names(ast)?;
    match lower(ast) {
        Err(e) if !e.is_resource() => return Err(e),
        _ => {}
    }
    lower(&crate::lang::optimize::optimize(ast))
}

/** Lower an AST into a VM program ending in `Halt`. */
pub fn compile(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    compile_full(ast).map(|c| c.ops)
}

/** Lower an AST without the optimizer, to check that optimization preserves behavior. */
pub fn compile_unoptimized(ast: &Ast) -> Result<Vec<Op>, CompileError> {
    check_names(ast)?;
    lower(ast).map(|c| c.ops)
}
