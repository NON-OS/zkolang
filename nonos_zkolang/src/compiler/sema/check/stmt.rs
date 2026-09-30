/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Statements (sections 8.1 to 8.3): `let`, `assert`, and an expression, whose value is
 * dropped after a `;` and must be `()` without one.
 */

use super::block_warn::discarded;
use super::cx::FnCx;
use crate::compiler::sema::ty::{TyKind, Types};
use crate::compiler::syntax::ast::{Stmt, StmtKind};
use crate::compiler::tir::TStmt;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The statement `s`; `diverges` becomes true if it leaves the block. */
    pub(super) fn stmt(&mut self, s: &'a Stmt, diverges: &mut bool) -> Option<TStmt> {
        match &s.kind {
            StmtKind::Let { pat, ty, init } => Some(self.let_stmt(pat, ty.as_ref(), init)),
            StmtKind::Assert { cond, message } => {
                let cond = self.expr(cond, Some(Types::BOOL));
                let message = message.clone();
                Some(TStmt::Assert { cond, message })
            }
            StmtKind::Expr { expr, semi } => {
                let e = match *semi {
                    true => self.infer(expr, None),
                    false => self.expr(expr, Some(Types::UNIT)),
                };
                if *semi && discarded(&e) {
                    self.discarded(e.span);
                }
                *diverges |= self.kind(e.ty) == TyKind::Never;
                Some(TStmt::Expr(e))
            }
            StmtKind::Empty => None,
        }
    }
}
