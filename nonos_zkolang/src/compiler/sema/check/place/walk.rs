/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Places (section 8.2): a mutable local, or a tuple field or indexed element of a place.
 * Anything else cannot be assigned or passed as `&mut`.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::{Expr, ExprKind, PathRoot};
use crate::compiler::tir::TPlace;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The place `e` names, of the type `want` if given. */
    pub(crate) fn place(&mut self, e: &'a Expr, want: Option<TyId>) -> TPlace {
        let mut steps: Vec<&'a Expr> = Vec::new();
        let mut at = e;
        let root = loop {
            match &at.kind {
                ExprKind::Paren(inner) => at = inner,
                ExprKind::TupleField(inner, _, _)
                | ExprKind::Field(inner, _)
                | ExprKind::Index(inner, _) => {
                    steps.push(at);
                    at = inner;
                }
                ExprKind::Path(p) => {
                    let bare_self = p.segments.is_empty() && p.root == PathRoot::SelfModule;
                    let name = p.as_ident().map(|i| i.name.as_str());
                    break name
                        .or(bare_self.then_some("self"))
                        .and_then(|n| self.lookup(n));
                }
                _ => break None,
            }
        };
        let Some(root) = root else {
            return self.not_a_place(e);
        };
        self.check_mutable(root, e);
        let mut ty = self
            .locals
            .get(root.0 as usize)
            .map_or(Types::ERROR, |l| l.ty);
        let mut proj = Vec::with_capacity(steps.len());
        for step in steps.into_iter().rev() {
            (ty, proj) = self.project(ty, step, proj);
        }
        if let Some(w) = want {
            if !self.unify(ty, w) {
                self.mismatch(e.span, w, ty);
            }
        }
        TPlace {
            root,
            proj,
            ty,
            span: e.span,
        }
    }
}
