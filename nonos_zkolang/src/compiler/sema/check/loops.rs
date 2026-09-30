/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Loops: `for` over a constant integer range or an array (section 8.6), and
 * `while cond limit N` (section 8.7). A loop's value is `()`.
 */

use alloc::boxed::Box;

use super::cx::FnCx;
use crate::compiler::sema::ty::Types;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Block, ConstArg, Expr, ForIter, Pattern};
use crate::compiler::tir::{TBlock, TExpr, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `for pat in iter { body }`. */
    pub(crate) fn for_expr(
        &mut self,
        pat: &'a Pattern,
        iter: &'a ForIter,
        body: &'a Block,
        at: Span,
    ) -> TExpr {
        self.push_scope();
        let kind = match iter {
            ForIter::Range { lo, hi, inclusive } => self.for_range(pat, lo, hi, *inclusive, body),
            ForIter::Array(a) => self.for_array(pat, a, false, body),
            ForIter::Enumerate(a) => self.for_array(pat, a, true, body),
        };
        self.pop_scope();
        TExpr {
            kind,
            ty: Types::UNIT,
            span: at,
        }
    }

    /** `while cond limit n { body }`. */
    pub(crate) fn while_expr(
        &mut self,
        cond: &'a Expr,
        limit: &'a ConstArg,
        body: &'a Block,
        at: Span,
    ) -> TExpr {
        let cond = self.expr(cond, Some(Types::BOOL));
        let limit = self.const_arg(limit).unwrap_or(0);
        let body = self.loop_body(body);
        TExpr {
            kind: TExprKind::While {
                cond: Box::new(cond),
                limit,
                body,
            },
            ty: Types::UNIT,
            span: at,
        }
    }

    /** A loop's body, which has type `()`. */
    pub(super) fn loop_body(&mut self, body: &'a Block) -> TBlock {
        self.loops = self.loops.saturating_add(1);
        let (b, ty) = self.block(body, Some(Types::UNIT));
        self.loops = self.loops.saturating_sub(1);
        if !self.fits(ty, Types::UNIT) {
            let span = b.tail.as_ref().map_or(b.span, |e| e.span);
            self.mismatch(span, Types::UNIT, ty);
        }
        b
    }
}
