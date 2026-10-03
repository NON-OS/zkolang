/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `if` (section 8.4). The branches of an `if` with an `else` share one type; without one,
 * the branch has type `()`. An `if` whose every branch leaves has type `!`. A branch
 * whose condition is constant and false is left out; one whose condition is constant
 * and true ends the `if`, the branches after it left out.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use super::if_arm::built;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Block, IfBranch};
use crate::compiler::tir::{TBlock, TExpr, TExprKind, TLit};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `if c1 { .. } else if c2 { .. } else { .. }`. */
    pub(crate) fn if_expr(
        &mut self,
        branches: &'a [IfBranch],
        last: Option<&'a Block>,
        want: Option<TyId>,
        at: Span,
    ) -> TExpr {
        let mut out: Vec<(TExpr, TBlock)> = Vec::with_capacity(branches.len());
        let mut ty = match last {
            Some(_) => want,
            None => Some(Types::UNIT),
        };
        let mut all_leave = true;
        let mut taken = None;
        for (i, br) in branches.iter().enumerate() {
            let (mark, errors) = (self.deferred.len(), self.sema.diags.error_count());
            let mut cond = self.expr(&br.cond, Some(Types::BOOL));
            let value = self.const_cond(&mut cond, mark, errors);
            if value == Some(false) {
                self.read_skipped(core::slice::from_ref(br), None);
                continue;
            }
            let blk = self.if_arm(&br.block, &mut ty, &mut all_leave);
            if value == Some(true) {
                self.read_skipped(branches.get(i + 1..).unwrap_or_default(), last);
                taken = Some(blk);
                break;
            }
            out.push((cond, blk));
        }
        let last = match (taken, last) {
            (Some(blk), None) => {
                let t = TExprKind::Lit(TLit::Bool(true));
                let cond = TExpr {
                    kind: t,
                    ty: Types::BOOL,
                    span: blk.span,
                };
                out.push((cond, blk));
                None
            }
            (Some(blk), Some(_)) => Some(blk),
            (None, last) => last.map(|b| self.if_arm(b, &mut ty, &mut all_leave)),
        };
        built(out, last, (ty, all_leave), at)
    }
}
