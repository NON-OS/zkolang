/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Guarded constraints (section 21.4): a condition that fails the run is conditioned on the
 * guard, so it holds trivially where the point does not run. An assertion of `e` becomes
 * `g -> e`, a range check of `x` checks `g ? x : 0`.
 */

use super::cx::Lower;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** Fail the run, where it runs, unless the boolean `ok` is 1. */
    pub(super) fn require(&mut self, ok: V) {
        let bad = self.b.not(ok);
        let guarded = self.b.mul(self.g, bad);
        self.b.emit(Inst::AssertZero(guarded));
    }

    /** Fail the run, where it runs, unless `x` is 0. */
    pub(super) fn require_zero(&mut self, x: V) {
        let guarded = self.b.mul(self.g, x);
        self.b.emit(Inst::AssertZero(guarded));
    }

    /** Fail the run, where it runs, unless `x < 2^n`. */
    pub(super) fn require_below(&mut self, x: V, n: u32) {
        let x = self.guarded(x, 0);
        self.b.emit(Inst::RangeCheck(x, n as u8));
    }

    /** `x` where the point runs, else the constant `safe`. */
    pub(super) fn guarded(&mut self, x: V, safe: i128) -> V {
        let s = self.b.konst(safe);
        self.b.sel(self.g, x, s)
    }

    /** `a && b` for booleans. */
    pub(super) fn and(&mut self, a: V, b: V) -> V {
        self.b.mul(a, b)
    }
}
