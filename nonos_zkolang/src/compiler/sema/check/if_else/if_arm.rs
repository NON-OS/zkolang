/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One branch of an `if` (section 8.4), whose type is the `if`'s unless it leaves; and the
 * `if` of the branches a constant condition keeps.
 */

use super::super::cx::FnCx;
use alloc::vec::Vec;

use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Block;
use crate::compiler::tir::{TBlock, TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * One branch, of type `ty` once one is known; `all_leave` stays true while every
     * branch so far has type `!`.
     */
    pub(super) fn if_arm(
        &mut self,
        b: &'a Block,
        ty: &mut Option<TyId>,
        all_leave: &mut bool,
    ) -> TBlock {
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

/**
 * The `if` of the branches `out` and the block `last`, of type `ty`, or `!` if it has a
 * last block and every branch leaves. With one block kept alone, it is that block.
 */
pub(super) fn built(
    out: Vec<(TExpr, TBlock)>,
    last: Option<TBlock>,
    (ty, all_leave): (Option<TyId>, bool),
    at: Span,
) -> TExpr {
    let ty = match (last.is_some() && all_leave, ty) {
        (true, _) => Types::NEVER,
        (false, Some(t)) => t,
        (false, None) => Types::UNIT,
    };
    let kind = match (out.is_empty(), last) {
        (true, Some(b)) => TExprKind::Block(b),
        (true, None) => TExprKind::Lit(TLit::Unit),
        (false, last) => TExprKind::If(out, last),
    };
    TExpr { kind, ty, span: at }
}
