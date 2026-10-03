/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The warning of a function of the program's own crate whose rows, with the functions
 * inlined into it, are more than half of what a proof holds (section 15.3, W0102).
 */

use alloc::format;

use super::build::MAX_ROWS;
use super::built::Built;
use super::cost::cost;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::Span;

/** The W0102 warnings of `b`, each at the function's header. */
pub(super) fn cost_warnings(b: &Built) -> Diagnostics {
    let mut out = Diagnostics::new();
    for (f, k) in &cost(b).fns {
        let i = f.0 as usize;
        let quiet = b.program.cost_quiet.get(i).copied().unwrap_or(true);
        let Some(tf) = b
            .program
            .fns
            .get(i)
            .filter(|_| k.inclusive > MAX_ROWS / 2 && !quiet)
        else {
            continue;
        };
        let at = Span::new(tf.span.file, tf.span.lo, tf.body.span.lo.max(tf.span.lo));
        let what = format!(
            "`{}` takes {} rows with the functions inlined into it, more than half of the {MAX_ROWS} a proof holds",
            tf.name, k.inclusive
        );
        let d = Diagnostic::warning(Code::COST_FUNCTION, what, at, "defined here");
        out.push(d.with_help(
            "each call writes its body out again; `zkolang check --cost` shows where the rows go, and the attribute `allow(cost)` on the function silences this",
        ));
    }
    out
}
