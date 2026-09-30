/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The bound on checks opened one inside another: a constant that needs a function that
 * needs a constant, and so on. Each keeps its state on the host stack, so their depth is
 * bounded.
 */

use super::cx::Sema;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

/** How many on-demand checks may be open at once. */
const MAX_DEPTH: u32 = 64;

impl<'a> Sema<'a> {
    /** Run `f` as a check opened inside the current one, needed at `at`. */
    pub(crate) fn within<T>(
        &mut self,
        at: Span,
        f: impl FnOnce(&mut Self) -> Option<T>,
    ) -> Option<T> {
        if self.depth >= MAX_DEPTH {
            let d = Diagnostic::error(
                Code::CONST_DEPTH,
                "constants and functions needed here depend on one another too deeply",
                at,
                "more than 64 deep",
            );
            self.diags.push(d);
            return None;
        }
        self.depth = self.depth.saturating_add(1);
        let out = f(self);
        self.depth = self.depth.saturating_sub(1);
        out
    }
}
