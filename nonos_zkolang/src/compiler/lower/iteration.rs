/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One iteration of a loop's body, and `while`: the guard after an iteration is its end
 * guard plus the guards of the `continue`s met in it.
 */

use super::cx::Lower;
use super::error::L;
use crate::compiler::ssa::V;
use crate::compiler::tir::{TBlock, TExpr};

impl<'p> Lower<'p> {
    /** One iteration of `body`. */
    pub(super) fn iteration(&mut self, body: &TBlock) -> L<()> {
        let zero = self.b.konst(0);
        if let Some(l) = self.innermost() {
            l.cont = zero;
        }
        self.block(body)?;
        let cont = self.innermost().map_or(zero, |l| l.cont);
        self.g = self.b.add(self.g, cont);
        Ok(())
    }

    /** `while cond limit n { body }`: the guard of the paths that stopped. */
    pub(super) fn while_(&mut self, cond: &TExpr, limit: u32, body: &TBlock) -> L<V> {
        let mut stopped = self.b.konst(0);
        for _ in 0..limit {
            if self.dead() {
                break;
            }
            let c = self.expr(cond)?.first().copied().unwrap_or(stopped);
            let run = self.and(self.g, c);
            let stop = self.b.sub(self.g, run);
            stopped = self.b.add(stopped, stop);
            self.g = run;
            self.iteration(body)?;
        }
        /* A run still going after the limit fails (section 8.7). */
        if !self.dead() {
            let c = self.expr(cond)?.first().copied().unwrap_or(stopped);
            let nc = self.b.not(c);
            self.require(nc);
        }
        let g = self.g;
        self.g = self.b.konst(0);
        Ok(self.b.add(stopped, g))
    }
}
