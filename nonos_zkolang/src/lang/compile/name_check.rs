/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Refuse the names the edition leaves ambiguous before lowering: a second function or
 * constant of one name, a binding named like a constant, and a binding in a loop body named
 * like the loop's variable. The lowering and the optimizer resolved these differently.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::block_binds::{block_binds, conflicting};
use crate::lang::parse::{Ast, Stmt};
use crate::lang::{CompileError, NameError};

/** Check a program's names, on the tree as written, before the optimizer rewrites it. */
pub(crate) fn check_names(ast: &Ast) -> Result<(), CompileError> {
    let dup = |n: &String| CompileError::Name(NameError::Duplicate { name: n.clone() });
    if let Some(n) = conflicting(ast.fns.iter().map(|f| (&f.name, (&f.params, &f.body)))) {
        return Err(dup(n));
    }
    if let Some(n) = conflicting(ast.consts.iter().map(|c| (&c.name, (&c.values, c.scalar)))) {
        return Err(dup(n));
    }
    let consts: Vec<&str> = ast.consts.iter().map(|c| c.name.as_str()).collect();
    for f in &ast.fns {
        for p in &f.params {
            bind(p, &consts, &[])?;
        }
        block_binds(&f.body, &mut |n| bind(n, &consts, &[]))?;
    }
    stmts(&ast.stmts, &consts, &mut Vec::new())
}

fn stmts(list: &[Stmt], consts: &[&str], loops: &mut Vec<String>) -> Result<(), CompileError> {
    for s in list {
        match s {
            Stmt::Let(n, e) => {
                block_binds(e, &mut |m| bind(m, consts, loops))?;
                bind(n, consts, loops)?;
            }
            Stmt::LetTuple(ns, e) => {
                block_binds(e, &mut |m| bind(m, consts, loops))?;
                for n in ns.iter().filter(|n| *n != "_") {
                    bind(n, consts, loops)?;
                }
            }
            Stmt::Input(n) | Stmt::Secret(n) => bind(n, consts, loops)?,
            Stmt::Output(e) | Stmt::Assert(e) => block_binds(e, &mut |m| bind(m, consts, loops))?,
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
