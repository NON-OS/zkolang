/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An arm of a `match`: its pattern binds names that its guard, a `bool`, and its value
 * see, and nothing after it does.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::Arm;
use crate::compiler::tir::{Labels, TArm};

impl<'s, 'a> FnCx<'s, 'a> {
    /**
     * The arm `arm` for a scrutinee of type `scrut`, its value of type `ty` once one is
     * known; `all_leave` stays true while every arm so far has type `!`.
     */
    pub(super) fn arm(
        &mut self,
        arm: &'a Arm,
        scrut: TyId,
        ty: &mut Option<TyId>,
        all_leave: &mut bool,
    ) -> TArm {
        self.push_scope();
        let errors = self.sema.diags.error_count();
        self.pats.arm = true;
        let pat = self.bind_pat(&arm.pat, scrut, &Labels::default(), &mut Vec::new());
        self.pats.arm = false;
        if self.sema.diags.error_count() > errors {
            self.pats.broken.push(arm.pat.span);
        }
        let guard = arm.guard.as_ref().map(|g| self.expr(g, Some(Types::BOOL)));
        let body = self.infer(&arm.body, *ty);
        if self.kind(body.ty) != TyKind::Never {
            *all_leave = false;
            match *ty {
                Some(w) if !self.unify(body.ty, w) => self.mismatch(body.span, w, body.ty),
                Some(_) => {}
                None => *ty = Some(body.ty),
            }
        }
        self.pop_scope();
        TArm {
            pat,
            guard,
            body,
            span: arm.pat.span,
        }
    }
}
