/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `if` (section 8.4). The branches of an `if` with an `else` share one type; without one,
 * the branch has type `()`. An `if` whose every branch leaves has type `!`.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Block, IfBranch};
use crate::compiler::tir::{TBlock, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `if c1 { .. } else if c2 { .. } else { .. }`. */
    pub(crate) fn if_expr(
        &mut self,
        branches: &'a [IfBranch],
        last: Option<&'a Block>,
        want: Option<TyId>,
        at: Span,
    ) -> TExpr {
        let mut out: Vec<(TExpr, TBlock)> = Vec::with_capacity(branches.len());
        let mut ty = match last {
            Some(_) => want,
            None => Some(Types::UNIT),
        };
        let mut all_leave = true;
        for br in branches {
            let cond = self.expr(&br.cond, Some(Types::BOOL));
            let blk = self.if_arm(&br.block, &mut ty, &mut all_leave);
            out.push((cond, blk));
        }
        let last = last.map(|b| self.if_arm(b, &mut ty, &mut all_leave));
        let ty = match (last.is_some() && all_leave, ty) {
            (true, _) => Types::NEVER,
            (false, Some(t)) => t,
            (false, None) => Types::UNIT,
        };
        TExpr {
            kind: TExprKind::If(out, last),
            ty,
            span: at,
        }
    }

    /**
     * One branch, of type `ty` once one is known; `all_leave` stays true while every
     * branch so far has type `!`.
     */
    fn if_arm(&mut self, b: &'a Block, ty: &mut Option<TyId>, all_leave: &mut bool) -> TBlock {
        let (blk, t) = self.block(b, *ty);
        if self.kind(t) == TyKind::Never {
            return blk;
        }
        *all_leave = false;
        match *ty {
            Some(w) if !self.unify(t, w) => {
                let span = blk.tail.as_ref().map_or(blk.span, |e| e.span);
                self.mismatch(span, w, t);
            }
            Some(_) => {}
            None => *ty = Some(t),
        }
        blk
    }
}
