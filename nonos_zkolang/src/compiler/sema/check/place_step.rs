/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One step into a place: a tuple field, or an element at an index. */

use alloc::format;
use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{LocalId, Proj, TPlace};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report that `e` is not a place (E0305), and check it for its own errors. */
    pub(super) fn not_a_place(&mut self, e: &'a Expr) -> TPlace {
        let help =
            "a place is a `let mut` variable, a `&mut` parameter, or a field or element of one";
        let d = Diagnostic::error(
            Code::NOT_A_PLACE,
            "this is not a place",
            e.span,
            "cannot be assigned",
        )
        .with_help(help);
        self.sema.diags.push(d);
        self.infer(e, None);
        let (root, proj, ty, span) = (LocalId(0), Vec::new(), Types::ERROR, e.span);
        TPlace {
            root,
            proj,
            ty,
            span,
        }
    }

    /** Take the step `step` into a value of type `ty`. */
    pub(super) fn project(
        &mut self,
        ty: TyId,
        step: &'a Expr,
        mut proj: Vec<Proj>,
    ) -> (TyId, Vec<Proj>) {
        let next = match (&step.kind, self.kind(ty)) {
            (ExprKind::TupleField(_, i, _), TyKind::Tuple(ts)) if (*i as usize) < ts.len() => {
                proj.push(Proj::TupleField(*i));
                ts.get(*i as usize).copied()
            }
            (ExprKind::Index(_, i), TyKind::Array(el, _)) => {
                proj.push(Proj::Index(self.expr(i, Some(Types::int(IntTy::Usize)))));
                Some(el)
            }
            (_, TyKind::Error) => None,
            _ => {
                let what = match &step.kind {
                    ExprKind::TupleField(_, i, _) => format!("{i}"),
                    _ => alloc::string::String::from("[]"),
                };
                self.no_field(ty, &what, step.span);
                None
            }
        };
        (next.unwrap_or(Types::ERROR), proj)
    }
}
