/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Fold the expressions inside a statement, recursing into a loop body.

use alloc::vec::Vec;

use super::expr::fold;
use crate::lang::parse::Stmt;

pub(super) fn fold_stmt(s: &Stmt) -> Stmt {
    match s {
        Stmt::Let(n, e) => Stmt::Let(n.clone(), fold(e)),
        Stmt::Assert(e) => Stmt::Assert(fold(e)),
        Stmt::Input(n) => Stmt::Input(n.clone()),
        Stmt::Secret(n) => Stmt::Secret(n.clone()),
        Stmt::Output(e) => Stmt::Output(fold(e)),
        Stmt::For { var, lo, hi, body } => Stmt::For {
            var: var.clone(),
            lo: *lo,
            hi: *hi,
            body: body.iter().map(fold_stmt).collect::<Vec<_>>(),
        },
    }
}
