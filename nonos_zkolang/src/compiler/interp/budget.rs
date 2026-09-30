/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The bounds on every run: a budget of steps, and a bound on how deep evaluation recurses. */

use super::machine::{fail, Flow, Interp};
use super::FailKind;
use crate::compiler::source::Span;

/** How deeply evaluation may recurse, through calls and nested expressions together. */
const MAX_DEPTH: u32 = 512;

impl<'e> Interp<'e> {
    /** Count one step at `at`, and enter one level of recursion. */
    pub(super) fn enter(&mut self, at: Span) -> Result<(), Flow> {
        self.steps = self.steps.saturating_add(1);
        self.depth = self.depth.saturating_add(1);
        if self.steps > self.budget || self.depth > MAX_DEPTH {
            return Err(fail(FailKind::Budget, at));
        }
        Ok(())
    }

    /** Leave one level of recursion. */
    pub(super) fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    /** Count `n` more steps at `at`, for work that allocates, such as `[e; n]`. */
    pub(super) fn spend(&mut self, n: u64, at: Span) -> Result<(), Flow> {
        self.steps = self.steps.saturating_add(n);
        if self.steps > self.budget {
            return Err(fail(FailKind::Budget, at));
        }
        Ok(())
    }
}
