/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The optimizer: a behaviour-preserving pass over the tree before lowering. It folds
//! constant sub-expressions, removes the algebraic identities that cost a trace row for
//! nothing, and propagates constant bindings, inlining them and freeing their registers.
//! Function bodies are folded before they inline. The proof a program produces is
//! unchanged; the trace it needs is smaller and its register pressure lower.

mod expr;
mod propagate;

use alloc::vec::Vec;

use super::parse::{Ast, FnDef};
use expr::fold;
use propagate::propagate;

/// Fold constants, drop no-op arithmetic, and propagate constants across a program.
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
        stmts: propagate(&ast.stmts),
    }
}
