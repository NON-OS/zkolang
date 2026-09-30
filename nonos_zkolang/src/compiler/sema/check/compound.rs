/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Tuple and array expressions (section 7.11). */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `(a, b, ...)`. */
    pub(crate) fn tuple(&mut self, es: &'a [Expr], want: Option<TyId>, at: Span) -> TExpr {
        let wants = match want.map(|w| self.kind(w)) {
            Some(TyKind::Tuple(ts)) if ts.len() == es.len() => ts,
            _ => Vec::new(),
        };
        let items: Vec<TExpr> = es
            .iter()
            .enumerate()
            .map(|(i, e)| self.infer(e, wants.get(i).copied()))
            .collect();
        let ty = self
            .sema
            .types
            .intern(TyKind::Tuple(items.iter().map(|e| e.ty).collect()));
        TExpr {
            kind: TExprKind::Tuple(items),
            ty,
            span: at,
        }
    }

    /** `[a, b, ...]`: every element has the type of the first. */
    pub(crate) fn array(&mut self, es: &'a [Expr], want: Option<TyId>, at: Span) -> TExpr {
        let elem_want = self.element_want(want);
        let Some((first, rest)) = es.split_first() else {
            let Some(el) = elem_want else {
                let d = Diagnostic::error(
                    Code::CANNOT_INFER,
                    "the type of an empty array is not known",
                    at,
                    "an empty array",
                )
                .with_help("write the array's type where it is bound: `let a: [u8; 0] = [];`");
                self.sema.diags.push(d);
                return self.error(at);
            };
            let ty = self.sema.types.intern(TyKind::Array(el, 0));
            return TExpr {
                kind: TExprKind::Array(Vec::new()),
                ty,
                span: at,
            };
        };
        let first = self.infer(first, elem_want);
        let el = first.ty;
        let mut items = alloc::vec![first];
        items.extend(rest.iter().map(|e| self.expr(e, Some(el))));
        let n = u32::try_from(items.len()).unwrap_or(u32::MAX);
        let ty = self.sema.types.intern(TyKind::Array(el, n));
        TExpr {
            kind: TExprKind::Array(items),
            ty,
            span: at,
        }
    }
}
