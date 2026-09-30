/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Loops. The body is walked until the labels of the locals and of the loop's exits stop
 * changing, then once more to report. Each iteration of a `while` runs under its
 * condition, and every iteration after a `break`, `continue` or `return` under that
 * exit's guard; the locals the loop changes take both labels, since how often it ran
 * depends on them. After a loop a `return` in it left, the rest runs under its guard.
 */

use super::flow::Flow;
use super::shape::Shape;
use super::taint::Taint;
use crate::compiler::tir::TExpr;

/** How many passes a loop's walk may take; the labels settle long before. */
const PASSES: usize = 300;

impl<'p> Flow<'p> {
    /** Walk the loop `e`. */
    pub(super) fn loop_(&mut self, e: &TExpr) {
        let (pc0, entry, report, ret0) = (self.pc, self.env.clone(), self.report, self.ret_guard);
        self.exits.push(Taint::PUBLIC);
        self.report = false;
        let mut cond = Taint::PUBLIC;
        for _ in 0..PASSES {
            let (before, exits) = (self.env.clone(), self.exits.last().copied());
            cond = cond.join(self.iteration(e, pc0));
            if self.settle_env(&before) && self.exits.last().copied() == exits {
                break;
            }
        }
        self.report = report;
        let before = self.env.clone();
        cond = cond.join(self.iteration(e, pc0));
        self.settle_env(&before);
        let exit = self.exits.pop().unwrap_or(Taint::SECRET).join(cond);
        for (now, was) in self.env.iter_mut().zip(&entry) {
            if now != was {
                *now = now.raised(exit);
            }
        }
        self.pc = match self.ret_guard != ret0 {
            true => pc0.join(self.ret_guard),
            false => pc0,
        };
    }

    /** Join the labels after a pass with those before it; say whether nothing changed. */
    fn settle_env(&mut self, before: &[Shape]) -> bool {
        let joined: alloc::vec::Vec<Shape> = before
            .iter()
            .zip(&self.env)
            .map(|(a, b)| a.join(b))
            .collect();
        let settled = joined.as_slice() == before;
        self.env = joined;
        settled
    }
}
