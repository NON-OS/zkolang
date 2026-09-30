/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Visiting the expressions of a block, and the index a place step evaluates. */

use super::{Proj, TBlock, TExpr, TStmt};

impl TBlock {
    /** Call `f` on each expression of the block's statements, then on its tail. */
    pub fn each_expr(&self, f: &mut dyn FnMut(&TExpr)) {
        for s in &self.stmts {
            match s {
                TStmt::Let { init, .. } => f(init),
                TStmt::Assert { cond, .. } => f(cond),
                TStmt::Expr(e) => f(e),
            }
        }
        if let Some(t) = &self.tail {
            f(t);
        }
    }
}

/** Call `f` on the index a place step evaluates, if it has one. */
pub(super) fn each_index(p: &Proj, f: &mut dyn FnMut(&TExpr)) {
    if let Proj::Index(i) = p {
        f(i);
    }
}
