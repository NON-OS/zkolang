/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The optimizer: a behaviour-preserving pass over the tree before lowering. It folds
//! constant sub-expressions and removes the algebraic identities that cost a trace row
//! for nothing, in the statements and in the function bodies that will be inlined into
//! them. The proof a program produces is unchanged; the trace it needs is smaller.

mod expr;
mod stmt;

use alloc::vec::Vec;

use super::parse::{Ast, FnDef};
use expr::fold;
use stmt::fold_stmt;

/// Fold constants and drop no-op arithmetic across a whole program.
pub(super) fn optimize(ast: &Ast) -> Ast {
    let fns = ast
        .fns
        .iter()
        .map(|f| FnDef {
            name: f.name.clone(),
            params: f.params.clone(),
            body: fold(&f.body),
        })
        .collect::<Vec<_>>();
    Ast {
        consts: ast.consts.clone(),
        fns,
        stmts: ast.stmts.iter().map(fold_stmt).collect(),
    }
}
