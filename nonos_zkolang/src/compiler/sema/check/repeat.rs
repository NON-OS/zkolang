/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `[e; n]` (section 7.11), and the element type an expected array type gives. */

use alloc::boxed::Box;

use super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{ConstArg, Expr};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `[v; n]`, `n` a constant `usize`. */
    pub(crate) fn repeat(
        &mut self,
        v: &'a Expr,
        n: &'a ConstArg,
        want: Option<TyId>,
        at: Span,
    ) -> TExpr {
        let elem_want = self.element_want(want);
        let v = self.infer(v, elem_want);
        let Some(n) = self.const_arg(n) else {
            return self.error(at);
        };
        let ty = self.sema.types.intern(TyKind::Array(v.ty, n));
        TExpr {
            kind: TExprKind::Repeat(Box::new(v), n),
            ty,
            span: at,
        }
    }

    /** The element type an array expected as `want` has. */
    pub(super) fn element_want(&mut self, want: Option<TyId>) -> Option<TyId> {
        match want.map(|w| self.kind(w)) {
            Some(TyKind::Array(el, _)) => Some(el),
            _ => None,
        }
    }
}
