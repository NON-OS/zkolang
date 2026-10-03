/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Tuple fields and array elements (sections 7.6 and 7.7). */

use alloc::boxed::Box;

use super::cx::FnCx;
use super::structs::Key;
use crate::compiler::sema::ty::{TyKind, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `a.i` on a tuple or tuple struct. */
    pub(crate) fn tuple_field(&mut self, a: &'a Expr, i: u32, at: Span) -> TExpr {
        let a = self.infer(a, None);
        let ty = self
            .field_of(a.ty, Key::Pos(i), at)
            .map_or(Types::ERROR, |f| f.1);
        TExpr {
            kind: TExprKind::TupleField(Box::new(a), i),
            ty,
            span: at,
        }
    }

    /** `a[i]` on an array, `i` a `usize`. */
    pub(crate) fn index(&mut self, a: &'a Expr, i: &'a Expr, at: Span) -> TExpr {
        let a = self.infer(a, None);
        let i = self.expr(i, Some(Types::int(IntTy::Usize)));
        let ty = match self.kind(a.ty) {
            TyKind::Array(el, _) => el,
            TyKind::Error => Types::ERROR,
            _ => {
                self.no_operator("[]", a.ty, at);
                Types::ERROR
            }
        };
        TExpr {
            kind: TExprKind::Index(Box::new(a), Box::new(i)),
            ty,
            span: at,
        }
    }
}
