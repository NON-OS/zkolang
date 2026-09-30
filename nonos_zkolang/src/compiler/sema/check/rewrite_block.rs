/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Rewriting the blocks and places of a settled body. */

use super::cx::FnCx;
use crate::compiler::sema::ty::TyId;
use crate::compiler::tir::{Proj, TBlock, TExpr, TStmt};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Rewrite a block. */
    pub(crate) fn rewrite_block(&mut self, b: &mut TBlock) {
        for s in b.stmts.iter_mut() {
            match s {
                TStmt::Let { init, .. } => self.rewrite(init),
                TStmt::Assert { cond, .. } => self.rewrite(cond),
                TStmt::Expr(e) => self.rewrite(e),
            }
        }
        if let Some(t) = &mut b.tail {
            self.rewrite(t);
        }
    }

    /** Rewrite the branches of an `if`, then its last block. */
    pub(super) fn rewrite_if(
        &mut self,
        branches: &mut [(TExpr, TBlock)],
        last: &mut Option<TBlock>,
    ) {
        for (c, b) in branches.iter_mut() {
            self.rewrite(c);
            self.rewrite_block(b);
        }
        if let Some(b) = last {
            self.rewrite_block(b);
        }
    }

    /** Rewrite a place's type and the indices along it. */
    pub(super) fn rewrite_place(&mut self, proj: &mut [Proj], ty: &mut TyId) {
        *ty = self.zonk(*ty, true);
        for p in proj.iter_mut() {
            if let Proj::Index(i) = p {
                self.rewrite(i);
            }
        }
    }
}
