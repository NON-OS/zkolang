/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `match` (section 8.5): the arms share one type, and a `match` whose every arm leaves
 * has type `!`. Whether the arms cover every value is checked once the types are known.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Arm, Expr};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `match scrut { arms }`, where a value of type `want` is expected, if any. */
    pub(crate) fn match_expr(
        &mut self,
        scrut: &'a Expr,
        arms: &'a [Arm],
        want: Option<TyId>,
        at: Span,
    ) -> TExpr {
        let s = self.infer(scrut, None);
        let mut ty = want;
        let mut all_leave = true;
        let mut out = Vec::with_capacity(arms.len());
        for arm in arms {
            out.push(self.arm(arm, s.ty, &mut ty, &mut all_leave));
        }
        let ty = match (all_leave, ty) {
            (true, _) => Types::NEVER,
            (false, Some(t)) => t,
            (false, None) => Types::UNIT,
        };
        TExpr {
            kind: TExprKind::Match(Box::new(s), out),
            ty,
            span: at,
        }
    }
}
