/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Prefix operators: `-` on a signed integer or a `field`, and `!` on a `bool` or an
 * integer; on a literal variable the check waits for the end of the body. And
 * `declassify`, which changes a label and not a type.
 */

use alloc::boxed::Box;

use super::cx::FnCx;
use super::deferred::Deferred;
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, UnOp};
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `-a` or `!a`. */
    pub(crate) fn unary(&mut self, op: UnOp, a: &'a Expr, want: Option<TyId>, at: Span) -> TExpr {
        let a = self.infer(a, want);
        let ty = self.numeric(a.ty);
        let ok = match (op, self.kind(ty)) {
            (_, TyKind::Error | TyKind::Never) => true,
            (UnOp::Neg, TyKind::Field) | (UnOp::Not, TyKind::Bool) => true,
            (UnOp::Neg, TyKind::Int(i)) => i.signed(),
            (UnOp::Not, TyKind::Int(_)) => true,
            (UnOp::Neg, TyKind::Var(_)) => {
                self.deferred.push(Deferred::Negatable { ty, span: at });
                true
            }
            (UnOp::Not, TyKind::Var(_)) => {
                self.deferred.push(Deferred::IntOnly {
                    ty,
                    span: at,
                    op: "!",
                });
                true
            }
            _ => false,
        };
        if !ok {
            self.no_operator(if op == UnOp::Neg { "-" } else { "!" }, ty, at);
        }
        TExpr {
            kind: TExprKind::Unary(op, Box::new(a)),
            ty,
            span: at,
        }
    }

    /** `declassify(v)`, of `v`'s type (section 7.12). */
    pub(crate) fn declassify(&mut self, v: &'a Expr, want: Option<TyId>, at: Span) -> TExpr {
        let v = self.infer(v, want);
        let ty = v.ty;
        TExpr {
            kind: TExprKind::Declassify(Box::new(v)),
            ty,
            span: at,
        }
    }
}
