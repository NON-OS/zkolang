/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking an expression: the dispatch over its kinds. `expr` checks against the type the
 * position expects; `infer` only lets that type guide literals and leaves the comparison
 * to its caller.
 */

use super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::tir::{TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Check `e` where a value of type `want` is expected, if any. */
    pub(crate) fn expr(&mut self, e: &'a Expr, want: Option<TyId>) -> TExpr {
        let t = self.infer(e, want);
        if let Some(w) = want {
            self.coerce(&t, w);
        }
        t
    }

    /** Check `e`, letting `want` guide the types of its literals. */
    pub(crate) fn infer(&mut self, e: &'a Expr, want: Option<TyId>) -> TExpr {
        let at = e.span;
        let mk = |kind, ty| TExpr { kind, ty, span: at };
        match &e.kind {
            ExprKind::Lit(l) => self.lit(l, want, at),
            ExprKind::Path(p) => self.path_expr(p, at),
            ExprKind::Unit => mk(TExprKind::Lit(TLit::Unit), Types::UNIT),
            ExprKind::Tuple(es) => self.tuple(es, want, at),
            ExprKind::Array(es) => self.array(es, want, at),
            ExprKind::Repeat(v, n) => self.repeat(v, n, want, at),
            ExprKind::Paren(inner) => {
                let t = self.infer(inner, want);
                TExpr { span: at, ..t }
            }
            ExprKind::Block(b) => {
                let (b, ty) = self.block(b, want);
                mk(TExprKind::Block(b), ty)
            }
            ExprKind::If(branches, last) => self.if_expr(branches, last.as_deref(), want, at),
            ExprKind::For { pat, iter, body } => self.for_expr(pat, iter, body, at),
            ExprKind::While { cond, limit, body } => self.while_expr(cond, limit, body, at),
            ExprKind::Return(v) => self.return_expr(v.as_deref(), at),
            ExprKind::Break | ExprKind::Continue => {
                self.loop_exit(matches!(e.kind, ExprKind::Break), at)
            }
            ExprKind::Declassify(v) => self.declassify(v, want, at),
            ExprKind::Unary(op, a) => self.unary(*op, a, want, at),
            ExprKind::Binary(first, links) => self.chain(first, links, want, at),
            ExprKind::Cast(a, ty) => self.cast(a, ty, at),
            ExprKind::Call(f, args) => self.call(f, args, at),
            ExprKind::MethodCall {
                receiver,
                method,
                generics,
                args,
            } => self.method_call(receiver, method, generics.is_some(), args, want, at),
            ExprKind::Field(a, name) => self.named_field(a, name, at),
            ExprKind::TupleField(a, i, _) => self.tuple_field(a, *i, at),
            ExprKind::Index(a, i) => self.index(a, i, at),
            ExprKind::Assign { op, place, value } => self.assign(*op, place, value, at),
            ExprKind::RefMut(_) => self.misplaced_ref_mut(at),
            ExprKind::Struct { path, fields } => self.struct_lit(path, fields, at),
            ExprKind::Match { scrutinee, arms } => self.match_expr(scrutinee, arms, want, at),
            ExprKind::Error => self.error(at),
        }
    }
}
