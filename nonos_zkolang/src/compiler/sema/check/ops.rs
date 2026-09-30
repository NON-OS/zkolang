/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Binary operators. A chain of one precedence is checked link by link, left to right: arithmetic
 * and bitwise operands share one type, a shift takes a `u32` count, `&&` and `||` take
 * `bool`, and a comparison compares two values of one type.
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{BinOp, Expr};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `first op1 e1 op2 e2 ...`, operators of one precedence. */
    pub(crate) fn chain(
        &mut self,
        first: &'a Expr,
        links: &'a [(BinOp, Expr)],
        want: Option<TyId>,
        at: Span,
    ) -> TExpr {
        let op0 = links.first().map_or(BinOp::Add, |l| l.0);
        let logic = matches!(op0, BinOp::And | BinOp::Or);
        let compare = matches!(
            op0,
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge
        );
        let first_want = if logic {
            Some(Types::BOOL)
        } else if compare {
            None
        } else {
            want
        };
        let first = if logic {
            self.expr(first, first_want)
        } else {
            self.infer(first, first_want)
        };
        let lhs = first.ty;
        let mut out = Vec::with_capacity(links.len());
        for (op, e) in links {
            let rhs_want = match op {
                BinOp::And | BinOp::Or => Types::BOOL,
                BinOp::Shl | BinOp::Shr => Types::int(IntTy::U32),
                _ => lhs,
            };
            let rhs = self.expr(e, Some(rhs_want));
            self.op_on(*op, lhs, at);
            out.push((*op, rhs));
        }
        let ty = if logic || compare {
            Types::BOOL
        } else {
            self.resolve(lhs)
        };
        TExpr {
            kind: TExprKind::Chain(Box::new(first), out),
            ty,
            span: at,
        }
    }
}
