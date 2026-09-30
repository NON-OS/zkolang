/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Loops, unrolled (sections 8.6 and 8.7). Each iteration runs under the guard the one
 * before ended with, plus its `continue`s; after the loop the `break`s join again. A
 * `while` iteration runs under its condition, and a run still going after `limit`
 * iterations fails. Unrolling stops early once the guard is the constant 0.
 */

use super::cx::{LoopCx, Lower};
use super::error::L;
use crate::compiler::ssa::Inst;
use crate::compiler::tir::{TExpr, TExprKind};

impl<'p> Lower<'p> {
    /** The loop `e`. */
    pub(super) fn loop_(&mut self, e: &TExpr) -> L<()> {
        let zero = self.b.konst(0);
        if let Some(f) = self.frame() {
            f.loops.push(LoopCx {
                broke: zero,
                cont: zero,
            });
        }
        let mut stopped = zero;
        match &e.kind {
            TExprKind::ForRange {
                var,
                lo,
                hi,
                inclusive,
                body,
            } => self.for_range(*var, (lo, hi, *inclusive), body)?,
            TExprKind::ForArray {
                index,
                pat,
                array,
                body,
            } => self.for_array(*index, pat, array, body)?,
            TExprKind::While { cond, limit, body } => stopped = self.while_(cond, *limit, body)?,
            _ => {}
        }
        let lp = self.frame().and_then(|f| f.loops.pop());
        let broke = lp.map_or(zero, |l| l.broke);
        let g = self.b.add(self.g, broke);
        self.g = self.b.add(g, stopped);
        Ok(())
    }

    /** Whether the guard is the constant 0: nothing after this point runs. */
    pub(super) fn dead(&self) -> bool {
        matches!(self.b.ssa.get(self.g), Some(Inst::Const(0)))
    }
}
