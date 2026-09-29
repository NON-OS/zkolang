/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Refuse the names the edition leaves ambiguous before lowering: a name defined twice, a
 * top-level binding named like a constant, and a binding in a loop body named like the
 * loop's variable, which the lowering and the optimizer resolved differently. A parameter
 * or a block local opens a scope of its own and shadows a constant of its name there.
 * Recursion is refused too, whether or not anything calls the function.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::block_binds::block_binds;
use super::duplicates::check_duplicates;
use super::recursion::recursive;
use crate::lang::parse::{Ast, Stmt};
use crate::lang::{CompileError, NameError};

/** Check a program's names, on the tree as written, before the optimizer rewrites it. */
pub(crate) fn check_names(ast: &Ast) -> Result<(), CompileError> {
    check_duplicates(ast)?;
    if let Some(n) = recursive(&ast.fns) {
        return Err(CompileError::Name(NameError::Recursive { name: n.clone() }));
    }
    let consts: Vec<&str> = ast.consts.iter().map(|c| c.name.as_str()).collect();
    stmts(&ast.stmts, &consts, &mut Vec::new())
}

fn stmts(list: &[Stmt], consts: &[&str], loops: &mut Vec<String>) -> Result<(), CompileError> {
    for s in list {
        match s {
            Stmt::Let(n, e) => {
                block_binds(e, &mut |m| bind(m, &[], loops))?;
                bind(n, consts, loops)?;
            }
            Stmt::LetTuple(ns, e) => {
                block_binds(e, &mut |m| bind(m, &[], loops))?;
                for n in ns.iter().filter(|n| *n != "_") {
                    bind(n, consts, loops)?;
                }
            }
            Stmt::Input(n) | Stmt::Secret(n) => bind(n, consts, loops)?,
            Stmt::Output(e) | Stmt::Assert(e) => block_binds(e, &mut |m| bind(m, &[], loops))?,
            Stmt::For { var, body, .. } => {
                loops.push(var.clone());
                stmts(body, consts, loops)?;
                loops.pop();
            }
        }
    }
    Ok(())
}

/** Refuse binding `name` where a constant or an enclosing loop's variable owns it. */
fn bind(name: &str, consts: &[&str], loops: &[String]) -> Result<(), CompileError> {
    let name = String::from(name);
    if consts.contains(&name.as_str()) {
        return Err(CompileError::Name(NameError::ShadowsConstant { name }));
    }
    if loops.contains(&name) {
        return Err(CompileError::Name(NameError::ShadowsLoopVariable { name }));
    }
    Ok(())
}
