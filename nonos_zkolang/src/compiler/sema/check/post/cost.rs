/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The cost warnings (section 15.3): a loop that unrolls to more iterations than the
 * unroll threshold (W0100), and a runtime index, read or written, into an array longer
 * than the dynamic index threshold (W0101), each in the program's own crate.
 */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::defs::Defs;
use crate::compiler::source::Span;
use crate::compiler::tir::TExpr;

impl<'s, 'a> FnCx<'s, 'a> {
    /** `for` over `lo..hi`, or `lo..=hi` if `inclusive`, at `at`. */
    pub(super) fn post_range(&mut self, (lo, hi): (&TExpr, &TExpr), inclusive: bool, at: Span) {
        let (Some(l), Some(h)) = (self.constant(lo), self.constant(hi)) else {
            return;
        };
        let n = h
            .int()
            .saturating_sub(l.int())
            .saturating_add(i128::from(inclusive));
        self.post_unroll(u64::try_from(n.max(0)).unwrap_or(u64::MAX), at);
    }

    /** Whether the body is of the program's own crate, not a dependency's or `std`'s. */
    pub(super) fn own(&self) -> bool {
        self.sema.defs.crate_of(self.module) == Defs::ROOT
    }

    /** A loop at `at` that unrolls to `n` iterations. */
    pub(super) fn post_unroll(&mut self, n: u64, at: Span) {
        let limit = self.sema.limits.unroll_warn;
        if n > limit && self.own() {
            let what = format!("this loop unrolls to {n} iterations, more than {limit}");
            let d = Diagnostic::warning(
                Code::COST_UNROLL,
                what,
                at,
                "every iteration is trace rows",
            )
            .with_help(
                "set `unroll_warn` in the manifest's `[cost]`, or put the attribute `allow(cost)` on the function",
            );
            self.sema.diags.push(d);
        }
    }
}
