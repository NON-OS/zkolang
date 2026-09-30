/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `for x in a..b`: the bounds are constants of one integer type, which `x` has
 * (section 8.6).
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::cx::FnCx;
use super::deferred::Deferred;
use crate::compiler::syntax::ast::{Block, Expr, Pattern};
use crate::compiler::tir::{Labels, TExprKind, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The loop over the range from `lo` to `hi`, which `inclusive` includes. */
    pub(super) fn for_range(
        &mut self,
        pat: &'a Pattern,
        lo: &'a Expr,
        hi: &'a Expr,
        inclusive: bool,
        body: &'a Block,
    ) -> TExprKind {
        let ty = self.vars.fresh(&mut self.sema.types, true);
        let (lo, hi) = (self.expr(lo, Some(ty)), self.expr(hi, Some(ty)));
        let span = lo.span.to(hi.span);
        self.deferred.push(Deferred::RangeInt { ty, span });
        let var = match self.bind_pat(pat, ty, &Labels::default(), &mut Vec::new()) {
            TPat::Bind(l) => l,
            TPat::Wild | TPat::Tuple(_) => {
                self.declare("_", ty, false, Labels::default(), pat.span)
            }
        };
        let body = self.loop_body(body);
        TExprKind::ForRange {
            var,
            lo: Box::new(lo),
            hi: Box::new(hi),
            inclusive,
            body,
        }
    }
}
