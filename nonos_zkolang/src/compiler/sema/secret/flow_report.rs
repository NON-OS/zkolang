/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What the walk reports: a secret at a public position, and a `declassify` of nothing. */

use super::flow::Flow;
use super::taint::Taint;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;

impl<'p> Flow<'p> {
    /**
     * Require `t`, the label of what reaches a public position at `at`, to be public: a
     * secret is an error (E0600); a slot's label becomes a requirement on callers.
     */
    pub(super) fn require_public(&mut self, t: Taint, at: Span, what: &str) {
        self.needs_public |= t.slots;
        if t.secret && self.report && self.reported.insert((at.lo, at.hi)) {
            let message = alloc::format!("a secret value reaches {what}");
            let d = Diagnostic::error(Code::SECRET_LEAK, message, at, "derived from a secret")
                .with_help("use `declassify(e)` if revealing this value is intended");
            self.found.push(d);
        }
    }

    /** Warn of `declassify` on a value that is public in every call (W0006). */
    pub(super) fn useless_declassify(&mut self, at: Span) {
        if self.report && self.reported.insert((at.lo, at.hi)) {
            let d = Diagnostic::warning(
                Code::USELESS_DECLASSIFY,
                "`declassify` of a public value",
                at,
                "already public",
            );
            self.found.push(d);
        }
    }
}
