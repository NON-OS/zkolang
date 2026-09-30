/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Assignment (section 8.2): `place = value` and `place op= value`, the place rooted in a
 * `let mut` variable or a `&mut` parameter.
 */

use alloc::boxed::Box;
use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::Types;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{AssignOp, BinOp, Expr};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{LocalId, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `place op value`. */
    pub(crate) fn assign(
        &mut self,
        op: AssignOp,
        place: &'a Expr,
        value: &'a Expr,
        at: Span,
    ) -> TExpr {
        let p = self.place(place, None);
        let (op, value) = match op {
            AssignOp::Assign => (None, self.expr(value, Some(p.ty))),
            AssignOp::Compound(b) => {
                let want = if matches!(b, BinOp::Shl | BinOp::Shr) {
                    Types::int(IntTy::U32)
                } else {
                    p.ty
                };
                let v = self.expr(value, Some(want));
                self.op_on(b, p.ty, at);
                (Some(b), v)
            }
        };
        TExpr {
            kind: TExprKind::Assign {
                place: p,
                op,
                value: Box::new(value),
            },
            ty: Types::UNIT,
            span: at,
        }
    }

    /** Report an assignment to a local not declared `let mut` (E0306). */
    pub(crate) fn check_mutable(&mut self, root: LocalId, e: &Expr) {
        let Some(l) = self.locals.get(root.0 as usize) else {
            return;
        };
        if !l.mutable {
            let (name, decl) = (l.name.clone(), l.span);
            let d = Diagnostic::error(
                Code::IMMUTABLE,
                format!("`{name}` is not declared `let mut`"),
                e.span,
                "cannot be changed",
            )
            .with_label(decl, "declared here")
            .with_help(format!("declare it with `let mut {name}`"));
            self.sema.diags.push(d);
        }
    }
}
