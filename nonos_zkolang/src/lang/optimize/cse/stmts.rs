/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Sharing across a statement list, one statement at a time. */

use alloc::vec::Vec;

use super::hoist::hoist;
use crate::lang::parse::Stmt;

/** Share repeated pure subexpressions across a statement list. */
pub(in crate::lang::optimize) fn cse(stmts: &[Stmt]) -> Vec<Stmt> {
    let mut ctr = 0usize;
    go(stmts, &mut ctr)
}

fn go(stmts: &[Stmt], ctr: &mut usize) -> Vec<Stmt> {
    let mut out = Vec::new();
    for s in stmts {
        match s {
            Stmt::Let(n, e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::Let(n.clone(), e2));
            }
            Stmt::LetTuple(names, e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::LetTuple(names.clone(), e2));
            }
            Stmt::Output(e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::Output(e2));
            }
            Stmt::Assert(e) => {
                let e2 = hoist(e.clone(), &mut out, ctr);
                out.push(Stmt::Assert(e2));
            }
            Stmt::Input(n) => out.push(Stmt::Input(n.clone())),
            Stmt::Secret(n) => out.push(Stmt::Secret(n.clone())),
            Stmt::For { var, lo, hi, body } => out.push(Stmt::For {
                var: var.clone(),
                lo: *lo,
                hi: *hi,
                body: go(body, ctr),
            }),
        }
    }
    out
}
